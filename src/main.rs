#![feature(int_to_from_bytes)]
extern crate png;

mod color;
mod geometry;

use std::u8;
use std::u16;
use std::fmt;
use std::error;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use png::HasParameters;
use color::*;

#[derive(Debug)]
enum WriteImageFileErr {
    File(io::Error),
    Encoding(png::EncodingError),
}

impl fmt::Display for WriteImageFileErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WriteImageFileErr::File(ref err) => write!(f, "File error: {}", err),
            WriteImageFileErr::Encoding(ref err) => write!(f, "Encoding error: {}", err),
        }
    }
}

impl error::Error for WriteImageFileErr {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WriteImageFileErr::File(ref err) => Some(err),
            WriteImageFileErr::Encoding(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for WriteImageFileErr {
    fn from(err: io::Error) -> WriteImageFileErr {
        WriteImageFileErr::File(err)
    }
}

impl From<png::EncodingError> for WriteImageFileErr {
    fn from(err: png::EncodingError) -> WriteImageFileErr {
        WriteImageFileErr::Encoding(err)
    }
}

const COLORS_PER_PIXEL: usize = 3;

fn save_image<'a>(file_name: &'a str, buffer: &ImageBuffer) -> Result<(), WriteImageFileErr> {
    let path = Path::new(file_name);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, buffer.imgx as u32, buffer.imgy as u32);

    encoder.set(png::ColorType::RGB).set(buffer.bit_depth);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(&buffer.buffer)?;

    Ok(())
}

#[derive(Clone)]
enum BytesPerColor {
    One = 1,
    Two = 2,
}

#[derive(Debug)]
struct ImageBuffer {
    pub bit_depth: png::BitDepth,
    pub buffer: Vec<u8>,
    pub bytes_per_pixel: usize,
    pub bytes_per_row: usize,
    pub imgx: usize,
    pub imgy: usize,
}

impl ImageBuffer {
    fn new(imgx: usize, imgy: usize, bytes_per_pixel: BytesPerColor) -> ImageBuffer {
        let bit_depth = match bytes_per_pixel {
            BytesPerColor::One => png::BitDepth::Eight,
            BytesPerColor::Two => png::BitDepth::Sixteen,
        };
        let bytes_per_pixel = COLORS_PER_PIXEL * bytes_per_pixel as usize;
        let bytes_per_row = bytes_per_pixel * imgx;
        let buffer: Vec<u8> = Vec::with_capacity(bytes_per_pixel * imgx * imgy);

        ImageBuffer {
            bit_depth: bit_depth,
            buffer: buffer,
            bytes_per_pixel: bytes_per_pixel,
            bytes_per_row: bytes_per_row,
            imgx: imgx,
            imgy: imgy,
        }
    }
    fn from_color_buffer(color_buffer: ColorBuffer, bytes_per_color: BytesPerColor) -> ImageBuffer {
        let mut buffer = ImageBuffer::new(color_buffer.imgx, color_buffer.imgy, bytes_per_color.clone());
        match bytes_per_color {
            BytesPerColor::Two => {
                let max = (u16::MAX as SamplePrecision) - 1e-6;
                for color in color_buffer.buffer {
                    let bytes = ((max * color) as u16).to_be_bytes();
                    buffer.buffer.extend(bytes.iter());
                }
            },
            BytesPerColor::One => {
                let max = (u8::MAX as SamplePrecision) - 1e-6;
                for color in color_buffer.buffer{
                    buffer.buffer.push((max * color) as u8);
                }
            },
        };
        buffer
    }
}

struct ColorBuffer {
    pub buffer: Vec<SamplePrecision>,
    pub imgx: usize,
    pub imgy: usize,
}

impl ColorBuffer {
    fn new(imgx: usize, imgy: usize) -> ColorBuffer {
        let buffer: Vec<SamplePrecision> = Vec::with_capacity(3 * imgx * imgy);

        ColorBuffer {
            buffer: buffer,
            imgx: imgx,
            imgy: imgy,
        }
    }
    fn push_color(&mut self, color: ColorSample) {
        self.buffer.extend([color.red, color.green, color.blue].iter());
    }
}

fn color_gradient_test(imgx: usize, imgy: usize) -> ColorBuffer {
    let mut color_buffer = ColorBuffer::new(imgx, imgy);
    for y in (0..imgy).rev() {
        for x in 0..imgx {
            let color = ColorSample {
                red: x as SamplePrecision / imgx as SamplePrecision,
                green: y as SamplePrecision / imgy as SamplePrecision,
                blue: 0.2,
            };
            color_buffer.push_color(color);
        }
    }
    color_buffer
}

fn draw_color_gradient() {
    let color_buffer = color_gradient_test(200, 100);
    let buffer = ImageBuffer::from_color_buffer(color_buffer, BytesPerColor::Two);

    save_image("images/001-color-gradient.png", &buffer).unwrap();
}

fn main() {
    draw_color_gradient();
}