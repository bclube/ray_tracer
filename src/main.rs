#![feature(int_to_from_bytes)]
extern crate png;

mod color;
mod geometry;
mod image;

use color::buffer::*;
use color::sample::*;
use geometry::ray::*;
use geometry::vec3::*;
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

fn color(ray: &Ray) -> ColorSample {
    let white = ColorSample {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    let light_blue = ColorSample {
        red: 0.5,
        green: 0.7,
        blue: 1.0,
    };
    let y = ray.direction.unit().y;
    let t = 0.5 * (y + 1.0);
    (1.0 - t) * white + t * light_blue
}

fn draw_empty_scene() {
    let imgx = 200;
    let imgy = 100;
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::ZERO;
    let mut color_buffer = ColorBuffer::new(imgx, imgy);
    for j in (0..imgy).rev() {
        let v = j as Dimension / imgy as Dimension;
        for i in 0..imgx {
            let u = i as Dimension / imgx as Dimension;
            let direction = lower_left + horizontal * u + vertical * v;
            let r = Ray {
                origin: origin,
                direction: direction,
            };
            let color = color(&r);
            color_buffer.push_color(color);
        }
    }
    let image_buffer = ImageBuffer::from_color_buffer(color_buffer, BytesPerColor::Two);
    save_image("images/003-empty-scene.png", &image_buffer).unwrap();
}

fn main() {
    draw_color_gradient();
    draw_empty_scene();
}
