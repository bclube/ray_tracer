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

fn hit_sphere(center: Vec3, radius: Dimension, ray: &Ray) -> Option<Vec3> {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        let t = (b + discriminant.sqrt()) / (-2.0 * a);
        let normal = ray.point_at_parameter(t) - center;
        Some(normal)
    } else {
        None
    }
}

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
    if let Some(normal) = hit_sphere(SPHERE_CENTER, radius, ray) {
        normal_to_color(normal)
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
