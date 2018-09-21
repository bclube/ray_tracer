use color::sample::*;
use geometry::ray::*;
use geometry::vec3::*;
use surface::material::*;

pub struct Metal {
    pub albedo: ColorSample,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_point: &Vec3, hit_normal: &Vec3) -> Option<HitResult> {
        let reflected = reflect(ray.direction.unit(), *hit_normal);
        let scattered = Ray {
            origin: *hit_point,
            direction: reflected,
        };
        if scattered.direction.dot(*hit_normal) > 0.0 {
            Some(HitResult {
                attenuation: self.albedo,
                scattered: scattered,
            })
        } else {
            None
        }
    }
}
