use color::sample::*;
use geometry::ray::*;
use geometry::vec3::*;
use surface::material::*;

pub struct Lambertian {
    pub albedo: ColorSample,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_point: &Vec3, hit_normal: &Vec3) -> Option<HitResult> {
        let target = *hit_point + *hit_normal + Vec3::random_in_unit_sphere();
        let scattered = Ray {
            origin: *hit_point,
            direction: target - *hit_point,
        };
        Some(HitResult {
            attenuation: self.albedo,
            scattered: scattered,
        })
    }
}
