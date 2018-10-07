use color::buffer::*;
use color::sample::*;
use image::convert::*;
use png;

const COLORS_PER_PIXEL: usize = 3;

#[derive(Copy, Clone)]
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
    pub fn from_color_buffer(
        color_buffer: ColorBuffer,
        bytes_per_color: BytesPerColor,
    ) -> ImageBuffer {
        let mut buffer = ImageBuffer::new(color_buffer.imgx, color_buffer.imgy, bytes_per_color);
        let sample_counts = color_buffer.sample_counts.iter().flat_map(|c| {
            let c = *c as SamplePrecision;
            vec![c, c, c]
        });
        let colors = color_buffer
            .buffer
            .iter()
            .zip(sample_counts)
            .map(|(c, n)| c / n)
            .map(|c| gamma_2(c));
        match bytes_per_color {
            BytesPerColor::Two => {
                for color in colors {
                    let bytes = to_sixteen_bits(color).to_be_bytes();
                    buffer.buffer.extend(bytes.iter());
                }
            }
            BytesPerColor::One => {
                for color in colors {
                    let byte = to_eight_bits(color);
                    buffer.buffer.push(byte);
                }
            }
        };
        buffer
    }
}
