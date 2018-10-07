#![feature(int_to_from_bytes)]
extern crate png;
extern crate rand;

mod camera;
mod color;
mod float_cmp;
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
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::*;
use std::sync::Arc;
use std::time::Duration;
use surface::dielectric::*;
use surface::lambertian::*;
use surface::material::*;
use surface::metal::*;
use world::bvh::*;
use world::entity::*;
use world::model::*;

fn color(ray: Ray, scene: &Arc<ModelSS>) -> ColorSample {
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

fn random_scene(imgx: usize, imgy: usize) -> (Arc<ModelSS>, Camera) {
    let mut rng = thread_rng();
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
    let mut spheres: Vec<Sphere> = Vec::new();
    let mut center_spheres: Vec<Box<ModelSS>> = Vec::new();
    // floor
    let sphere = Sphere {
        center: Vec3::new(0.0, -1e12, 0.0),
        radius: 1e12,
    };
    spheres.push(sphere);
    let floor = Box::new(WorldEntity {
        shape: Box::new(sphere),
        material: Arc::new(Lambertian {
            albedo: ColorSample {
                red: 0.5,
                green: 0.5,
                blue: 0.5,
            },
        }),
    });
    // dielectric
    let sphere = Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
    };
    spheres.push(sphere);
    center_spheres.push(Box::new(WorldEntity {
        shape: Box::new(sphere),
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    // lambertian
    let sphere = Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
    };
    spheres.push(sphere);
    center_spheres.push(Box::new(WorldEntity {
        shape: Box::new(sphere),
        material: Arc::new(Lambertian {
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
    center_spheres.push(Box::new(WorldEntity {
        shape: Box::new(sphere),
        material: Arc::new(Metal::new(
            ColorSample {
                red: 0.7,
                green: 0.6,
                blue: 0.5,
            },
            0.0,
        )),
    }));
    // random sphere field
    let mut sphere_field: Vec<Box<ModelSS>> = Vec::new();
    for a in -11..=11 {
        for b in -11..=11 {
            let material: Arc<MaterialSS> = match rng.gen_range::<Dimension>(0.0, 1.0) {
                v if v < 0.8 => Arc::new(Lambertian {
                    albedo: ColorSample {
                        red: rng.gen_range::<Dimension>(0.0, 1.0)
                            * rng.gen_range::<Dimension>(0.0, 1.0),
                        green: rng.gen_range::<Dimension>(0.0, 1.0)
                            * rng.gen_range::<Dimension>(0.0, 1.0),
                        blue: rng.gen_range::<Dimension>(0.0, 1.0)
                            * rng.gen_range::<Dimension>(0.0, 1.0),
                    },
                }),
                v if v < 0.95 => Arc::new(Metal::new(
                    ColorSample {
                        red: rng.gen_range::<Dimension>(0.5, 1.0),
                        green: rng.gen_range::<Dimension>(0.5, 1.0),
                        blue: rng.gen_range::<Dimension>(0.5, 1.0),
                    },
                    rng.gen_range::<Dimension>(0.0, 0.5),
                )),
                _ => Arc::new(Dielectric { ref_idx: 1.5 }),
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
            sphere_field.push(Box::new(WorldEntity {
                shape: Box::new(new_sphere.expect("unable to place sphere")),
                material: material,
            }));
        }
    }
    let sphere_field =
        Tree::from_list_on_dimensions(&mut sphere_field, &[SplitDim::X, SplitDim::Z]);
    let center_spheres = Tree::from_list_on_dimensions(&mut center_spheres, &[SplitDim::X]);
    let mut scene: Vec<Box<ModelSS>> = vec![floor, sphere_field, center_spheres];
    let scene = Arc::from(Tree::from_list_on_dimensions(&mut scene, &[SplitDim::Y]));
    (scene, camera)
}

fn render_scene() {
    let imgx = 600;
    let imgy = 400;
    let n_samples = 1000;
    'new_scene: loop {
        let (scene, camera) = random_scene(imgx, imgy);
        for (imgx, imgy, n_samples, n_threads) in vec![
            (imgx / 4, imgy / 4, 1, 1),
            (imgx, imgy, 1, 1),
            (imgx, imgy, n_samples, 15),
        ] {
            let (tx, rx) = sync_channel::<ColorBuffer>(n_threads * 3);
            let ct = Arc::new(AtomicUsize::new(0));
            for _ in 0..n_threads {
                let ct = ct.clone();
                let tx = tx.clone();
                let scene = scene.clone();
                let camera = camera.clone();
                std::thread::spawn(move || {
                    let mut rng = thread_rng();
                    loop {
                        let s = ct.fetch_add(1, Ordering::SeqCst);
                        if s >= n_samples {
                            break;
                        }
                        let mut color_buffer = ColorBuffer::new(imgx, imgy);
                        let ru = rng.gen_range::<Dimension>(0.0, 1.0);
                        let rv = rng.gen_range::<Dimension>(0.0, 1.0);
                        for j in (0..imgy).rev() {
                            let v = (rv + j as Dimension) / imgy as Dimension;
                            for i in 0..imgx {
                                let u = (ru + i as Dimension) / imgx as Dimension;
                                let ray = camera.get_ray(u, v);
                                let color = color(ray, &scene);
                                color_buffer.add_color(i, imgy - 1 - j, color);
                            }
                        }
                        tx.send(color_buffer).unwrap();
                    }
                });
            }

            drop(tx);

            let mut sample = 0usize;
            let mut color_buffer = ColorBuffer::new(imgx, imgy);
            loop {
                let mut buffer: ColorBuffer;
                match rx.recv() {
                    Ok(data) => buffer = data,
                    Err(_) => break,
                };
                loop {
                    sample += 1;
                    color_buffer.add_buffer(buffer);
                    match rx.recv_timeout(Duration::from_millis(500)) {
                        Ok(data) => buffer = data,
                        Err(_) => break,
                    }
                }

                println!("sample {}/{}", sample, n_samples);
                let image_buffer =
                    ImageBuffer::from_color_buffer(&color_buffer, BytesPerColor::Two);
                save_image("images/012-random-scene.png", &image_buffer).unwrap();
            }

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
