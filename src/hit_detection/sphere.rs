use geometry::ray::*;
use geometry::vec3::*;
use hit_detection::hitable::*;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: Dimension,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: Dimension, t_max: Dimension) -> Option<HitRecord> {
        hit_sphere(r, t_min, t_max, self.center, self.radius)
    }
}

impl Sphere {
    pub fn intersects(&self, other: &Sphere) -> bool {
        let center_distance = (self.center - other.center).length();
        let radius_sum = self.radius.abs() + other.radius.abs();
        radius_sum > center_distance
    }
}

pub fn hit_sphere(
    ray: &Ray,
    t_min: Dimension,
    t_max: Dimension,
    center: Vec3,
    radius: Dimension,
) -> Option<HitRecord> {
    let oc = ray.origin - center;
    let a = ray.direction.squared_length();
    let b = oc.dot(ray.direction);
    let c = oc.squared_length() - radius * radius;
    let discriminant = b * b - a * c;
    if discriminant <= 0.0 {
        return None;
    }

    let sqrt = discriminant.sqrt();

    for root in [-sqrt, sqrt].iter() {
        let temp = (-b + root) / a;
        if temp >= t_max {
            continue;
        }
        if temp <= t_min {
            continue;
        }
        let p = ray.point_at_parameter(temp);
        return Some(HitRecord {
            t: temp,
            p: p,
            normal: (p - center) / radius,
        });
    }

    None
}
