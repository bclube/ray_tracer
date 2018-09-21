use color::sample::*;
use geometry::ray::*;
use geometry::vec3::*;
use surface::material::*;

pub struct Metal {
    albedo: ColorSample,
    fuzz: Dimension,
}

impl Metal {
    pub fn new(albedo: ColorSample, fuzz: Dimension) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz.max(0.0).min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_point: &Vec3, hit_normal: &Vec3) -> Option<HitResult> {
        let reflected = reflect(ray.direction.unit(), *hit_normal);
        let direction = if self.fuzz == 0.0 {
            reflected
        } else {
            reflected + self.fuzz * Vec3::random_in_unit_sphere()
        };
        let scattered = Ray {
            origin: *hit_point,
            direction: direction,
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
