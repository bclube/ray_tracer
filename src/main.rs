#![feature(int_to_from_bytes)]
extern crate png;
extern crate rand;

mod camera;
mod color;
mod geometry;
mod hit_detection;
mod image;
mod surface;
mod world;

use camera::*;
use color::buffer::*;
use color::sample::*;
use geometry::ray::*;
use geometry::vec3::*;
use hit_detection::sphere::*;
use image::buffer::*;
use image::write::*;
use rand::{thread_rng, Rng};
use std::rc::*;
use surface::dielectric::*;
use surface::lambertian::*;
use surface::metal::*;
use world::entity::*;
use world::model::*;

fn normal_to_color(normal: Vec3) -> ColorSample {
    let half_unit_normal = normal.unit() * 0.5;
    ColorSample {
        red: half_unit_normal.x + 0.5,
        green: half_unit_normal.y + 0.5,
        blue: half_unit_normal.z + 0.5,
    }
}

fn color(ray: Ray, scene: &Model) -> ColorSample {
    let mut attenuation = ColorSample::WHITE;
    let mut new_ray = ray;
    for _depth in 0..50 {
        if let Some(hit) = scene.hit_model(&new_ray, 1e-3, MAX_DIMENSION) {
            if let Some(scatter_result) =
                hit.material
                    .scatter(&new_ray, &hit.hit_record.p, &hit.hit_record.normal)
            {
                attenuation *= scatter_result.attenuation;
                new_ray = scatter_result.scattered;
                continue;
            } else {
                break;
            }
        } else {
            const LIGHT_BLUE: ColorSample = ColorSample {
                red: 0.5,
                green: 0.7,
                blue: 1.0,
            };
            let y = ray.direction.unit().y;
            let t = 0.5 * (y + 1.0);
            let col = (1.0 - t) * ColorSample::WHITE + t * LIGHT_BLUE;
            return attenuation * col;
        }
    }
    ColorSample::BLACK
}

fn render_scene() {
    let imgx = 400;
    let imgy = 200;
    let n_samples = 5000;
    let look_from = Vec3::new(10.0, 0.3, -5.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let vert_fov_degrees = 9.0;
    let aspect = imgx as Dimension / imgy as Dimension;
    let aperture = 0.15;
    let distance_to_focus = (look_from - look_at).length();
    let camera = Camera::new(
        look_from,
        look_at,
        up,
        vert_fov_degrees,
        aspect,
        aperture,
        distance_to_focus,
    );
    let mut color_buffer = ColorBuffer::new(imgx, imgy);
    let scene: Vec<Box<Model>> = vec![
        Box::new(WorldEntity {
            shape: Box::new(Sphere {
                center: Vec3::new(0.0, -100_000.5, -1.0),
                radius: 100_000.0,
            }),
            material: Rc::new(Lambertian {
                albedo: ColorSample {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.5,
                },
            }),
        }),
        Box::new(WorldEntity {
            shape: Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            material: Rc::new(Dielectric { ref_idx: 1.5 }),
        }),
        Box::new(WorldEntity {
            shape: Box::new(Sphere {
                center: Vec3::new(2.0, 0.0, -1.0),
                radius: 0.5,
            }),
            material: Rc::new(Metal::new(
                ColorSample {
                    red: 0.8,
                    green: 0.6,
                    blue: 0.2,
                },
                0.3,
            )),
        }),
        Box::new(WorldEntity {
            shape: Box::new(Sphere {
                center: Vec3::new(-2.0, 0.0, -1.0),
                radius: 0.5,
            }),
            material: Rc::new(Lambertian {
                albedo: ColorSample {
                    red: 0.1,
                    green: 0.2,
                    blue: 0.5,
                },
            }),
        }),
    ];
    let mut rng = thread_rng();
    for j in (0..imgy).rev() {
        for i in 0..imgx {
            let mut color_sample = ColorSample::BLACK;
            for _ in 0..n_samples {
                let u = (rng.gen_range::<Dimension>(0.0, 1.0) + i as Dimension) / imgx as Dimension;
                let v = (rng.gen_range::<Dimension>(0.0, 1.0) + j as Dimension) / imgy as Dimension;
                let ray = camera.get_ray(u, v);
                color_sample += color(ray, &scene.as_slice());
            }
            color_buffer.push_color(color_sample / n_samples);
        }
    }
    let image_buffer = ImageBuffer::from_color_buffer(color_buffer, BytesPerColor::Two);
    save_image("images/011-depth-of-field.png", &image_buffer).unwrap();
}

fn main() {
    render_scene();
}
