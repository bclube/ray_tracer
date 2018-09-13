use color::buffer::*;
use color::sample::*;
use png;
use std::u16;
use std::u8;

const COLORS_PER_PIXEL: usize = 3;

#[derive(Clone)]
pub enum BytesPerColor {
    One = 1,
    Two = 2,
}

#[derive(Debug)]
pub struct ImageBuffer {
    pub bit_depth: png::BitDepth,
    pub buffer: Vec<u8>,
    pub bytes_per_pixel: usize,
    pub bytes_per_row: usize,
    pub imgx: usize,
    pub imgy: usize,
}

impl ImageBuffer {
    pub fn new(imgx: usize, imgy: usize, bytes_per_pixel: BytesPerColor) -> ImageBuffer {
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
    pub fn from_color_buffer(color_buffer: ColorBuffer, bytes_per_color: BytesPerColor) -> ImageBuffer {
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
