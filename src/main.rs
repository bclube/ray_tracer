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
use std::io::{self, BufRead};
use std::rc::*;
use surface::dielectric::*;
use surface::lambertian::*;
use surface::material::*;
use surface::metal::*;
use world::entity::*;
use world::model::*;

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
    let imgx = 600;
    let imgy = 400;
    let n_samples = 1000;
    let look_from = Vec3::new(20.0, 1.9, 5.0);
    let look_at = Vec3::new(0.0, 0.5, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let vert_fov_degrees = 10.0;
    let aspect = imgx as Dimension / imgy as Dimension;
    let aperture = 0.04;
    let distance_to_focus = (look_from - look_at).length() - 2.0;
    let camera = Camera::new(
        look_from,
        look_at,
        up,
        vert_fov_degrees,
        aspect,
        aperture,
        distance_to_focus,
    );
    'new_scene: loop {
        let mut spheres: Vec<Sphere> = Vec::new();
        let mut rng = thread_rng();
        let mut scene: Vec<Box<Model>> = Vec::new();
        // floor
        let sphere = Sphere {
            center: Vec3::new(0.0, -1e12, 0.0),
            radius: 1e12,
        };
        spheres.push(sphere);
        scene.push(Box::new(WorldEntity {
            shape: Box::new(sphere),
            material: Rc::new(Lambertian {
                albedo: ColorSample {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.5,
                },
            }),
        }));
        // dielectric
        let sphere = Sphere {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
        };
        spheres.push(sphere);
        scene.push(Box::new(WorldEntity {
            shape: Box::new(sphere),
            material: Rc::new(Dielectric { ref_idx: 1.5 }),
        }));
        // lambertian
        let sphere = Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
        };
        spheres.push(sphere);
        scene.push(Box::new(WorldEntity {
            shape: Box::new(sphere),
            material: Rc::new(Lambertian {
                albedo: ColorSample {
                    red: 0.4,
                    green: 0.2,
                    blue: 0.1,
                },
            }),
        }));
        // metal
        let sphere = Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
        };
        spheres.push(sphere);
        scene.push(Box::new(WorldEntity {
            shape: Box::new(sphere),
            material: Rc::new(Metal::new(
                ColorSample {
                    red: 0.7,
                    green: 0.6,
                    blue: 0.5,
                },
                0.0,
            )),
        }));
        // random sphere field
        for a in -11..=11 {
            for b in -11..=11 {
                let material: Rc<Material> = match rng.gen_range::<Dimension>(0.0, 1.0) {
                    v if v < 0.8 => Rc::new(Lambertian {
                        albedo: ColorSample {
                            red: rng.gen_range::<Dimension>(0.0, 1.0)
                                * rng.gen_range::<Dimension>(0.0, 1.0),
                            green: rng.gen_range::<Dimension>(0.0, 1.0)
                                * rng.gen_range::<Dimension>(0.0, 1.0),
                            blue: rng.gen_range::<Dimension>(0.0, 1.0)
                                * rng.gen_range::<Dimension>(0.0, 1.0),
                        },
                    }),
                    v if v < 0.95 => Rc::new(Metal::new(
                        ColorSample {
                            red: rng.gen_range::<Dimension>(0.5, 1.0),
                            green: rng.gen_range::<Dimension>(0.5, 1.0),
                            blue: rng.gen_range::<Dimension>(0.5, 1.0),
                        },
                        rng.gen_range::<Dimension>(0.0, 0.5),
                    )),
                    _ => Rc::new(Dielectric { ref_idx: 1.5 }),
                };
                let mut new_sphere: Option<Sphere> = None;
                for _ in 0..1000 {
                    let sphere = Sphere {
                        center: Vec3 {
                            x: a as Dimension + rng.gen_range::<Dimension>(0.0, 0.9),
                            y: 0.2,
                            z: b as Dimension + rng.gen_range::<Dimension>(0.0, 0.9),
                        },
                        radius: 0.2,
                    };
                    if !spheres.iter().any(|&s| s.intersects(&sphere)) {
                        spheres.push(sphere);
                        new_sphere = Some(sphere);
                        break;
                    }
                }
                scene.push(Box::new(WorldEntity {
                    shape: Box::new(new_sphere.expect("unable to place sphere")),
                    material: material,
                }));
            }
        }
        for (imgx, imgy, n_samples) in vec![
            (imgx / 4, imgy / 4, 1),
            (imgx / 2, imgy / 2, 10),
            (imgx, imgy, n_samples),
        ] {
            let mut color_buffer = ColorBuffer::new(imgx, imgy);
            for j in (0..imgy).rev() {
                for i in 0..imgx {
                    let mut color_sample = ColorSample::BLACK;
                    for _ in 0..n_samples {
                        let u = (rng.gen_range::<Dimension>(0.0, 1.0) + i as Dimension)
                            / imgx as Dimension;
                        let v = (rng.gen_range::<Dimension>(0.0, 1.0) + j as Dimension)
                            / imgy as Dimension;
                        let ray = camera.get_ray(u, v);
                        color_sample += color(ray, &scene.as_slice());
                    }
                    color_buffer.push_color(color_sample / n_samples);
                }
                println!("row {}/{}", j, imgy);
            }
            let image_buffer = ImageBuffer::from_color_buffer(color_buffer, BytesPerColor::Two);
            save_image("images/012-random-scene.png", &image_buffer).unwrap();

            println!("ok? ('yes' to use this world)");
            let stdin = io::stdin();
            let line = stdin.lock().lines().next().unwrap().unwrap();
            if line != "yes" {
                continue 'new_scene;
            }
        }
    }
}

fn main() {
    render_scene();
}
