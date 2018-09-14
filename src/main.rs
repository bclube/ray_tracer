#![feature(int_to_from_bytes)]
extern crate png;

mod color;
mod geometry;
mod hit_detection;
mod image;

use color::buffer::*;
use color::sample::*;
use geometry::ray::*;
use geometry::vec3::*;
use hit_detection::sphere::*;
use image::buffer::*;
use image::write::*;

fn normal_to_color(normal: Vec3) -> ColorSample {
    let half_unit_normal = normal.unit() * 0.5;
    ColorSample {
        red: half_unit_normal.x + 0.5,
        green: half_unit_normal.y + 0.5,
        blue: half_unit_normal.z + 0.5,
    }
}

fn color(ray: &Ray) -> ColorSample {
    const SPHERE_CENTER: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let radius = 0.5;
    if let Some(hit_record) = hit_sphere(ray, 0.0, MAX_DIMENSION, SPHERE_CENTER, radius) {
        normal_to_color(hit_record.normal)
    } else {
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
}

fn render_scene() {
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
    save_image("images/005-sphere-normal-test.png", &image_buffer).unwrap();
}

fn main() {
    render_scene();
}
