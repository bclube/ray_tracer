use color::sample::*;
use geometry::ray::*;
use geometry::vec3::*;
use rand::{thread_rng, Rng};
use surface::material::*;

pub struct Dielectric {
    pub ref_idx: Dimension,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_point: &Vec3, hit_normal: &Vec3) -> Option<HitResult> {
        let perpendicular = ray.direction.dot(*hit_normal);
        let (outward_normal, ni_over_nt, cosine) = if perpendicular > 0.0 {
            (
                -*hit_normal,
                self.ref_idx,
                self.ref_idx * perpendicular / ray.direction.length(),
            )
        } else {
            (
                *hit_normal,
                1.0 / self.ref_idx,
                -perpendicular / ray.direction.length(),
            )
        };
        if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
            let scl = schlick(cosine, self.ref_idx);
            let dir = if thread_rng().gen_range::<Dimension>(0.0, 1.0) < scl {
                reflect(ray.direction.unit(), *hit_normal)
            } else {
                refracted
            };
            Some(HitResult {
                attenuation: ColorSample::WHITE,
                scattered: Ray {
                    origin: *hit_point,
                    direction: dir,
                },
            })
        } else {
            Some(HitResult {
                attenuation: ColorSample::WHITE,
                scattered: Ray {
                    origin: *hit_point,
                    direction: reflect(ray.direction.unit(), *hit_normal),
                },
            })
        }
    }
}
