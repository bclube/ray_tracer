use color::sample::*;
use geometry::ray::*;
use geometry::vec3::*;

pub struct HitResult {
    pub attenuation: ColorSample,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_point: &Vec3, hit_normal: &Vec3) -> Option<HitResult>;
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}
