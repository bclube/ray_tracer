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

pub fn refract(v: Vec3, normal: Vec3, ni_over_nt: Dimension) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - normal * dt) - normal * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub fn schlick(cosine: Dimension, ref_idx: Dimension) -> Dimension {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r1 = r0 * r0;
    r1 + (1.0 + r1) * (1.0 - cosine).powi(5)
}
