#![feature(int_to_from_bytes)]
extern crate png;

mod color;
mod geometry;
mod image;

use color::buffer::*;
use color::sample::*;
use image::buffer::*;
use image::write::*;

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
