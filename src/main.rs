#![feature(int_to_from_bytes)]
extern crate png;
extern crate rand;

mod camera;
mod color;
mod geometry;
mod hit_detection;
mod image;

use camera::*;
use color::buffer::*;
use color::sample::*;
use geometry::ray::*;
use geometry::vec3::*;
use hit_detection::hitable::*;
use hit_detection::sphere::*;
use image::buffer::*;
use image::write::*;
use rand::{thread_rng, Rng};

fn normal_to_color(normal: Vec3) -> ColorSample {
    let half_unit_normal = normal.unit() * 0.5;
    ColorSample {
        red: half_unit_normal.x + 0.5,
        green: half_unit_normal.y + 0.5,
        blue: half_unit_normal.z + 0.5,
    }
}

fn color(ray: &Ray, scene: &Hitable) -> ColorSample {
    if let Some(hit_record) = scene.hit(ray, 0.0, MAX_DIMENSION) {
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
    let n_samples = 100;
    let camera = Camera {
        lower_left: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::ZERO,
    };
    let mut color_buffer = ColorBuffer::new(imgx, imgy);
    let scene: Vec<Box<Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
    ];
    let mut rng = thread_rng();
    for j in (0..imgy).rev() {
        let v = j as Dimension / imgy as Dimension;
        for i in 0..imgx {
            let mut color_sample = ColorSample::BLACK;
            for _ in 0..n_samples {
                let u = (rng.gen_range::<Dimension>(0.0, 1.0) + i as Dimension) / imgx as Dimension;
                let v = (rng.gen_range::<Dimension>(0.0, 1.0) + j as Dimension) / imgy as Dimension;
                let ray = camera.get_ray(u, v);
                color_sample += color(&ray, &scene.as_slice());
            }
            color_buffer.push_color(color_sample / n_samples);
        }
    }
    let image_buffer = ImageBuffer::from_color_buffer(color_buffer, BytesPerColor::Two);
    save_image("images/006-anti-aliasing.png", &image_buffer).unwrap();
}

fn main() {
    render_scene();
}
