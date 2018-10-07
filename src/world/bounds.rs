use geometry::ray::*;
use geometry::vec3::*;
use std::mem::swap;

#[derive(Copy, Clone)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds {
    pub fn new(v1: Vec3, v2: Vec3) -> Bounds {
        let min_x = v1.x.min(v2.x);
        let max_x = v1.x.max(v2.x);
        let min_y = v1.y.min(v2.y);
        let max_y = v1.y.max(v2.y);
        let min_z = v1.z.min(v2.z);
        let max_z = v1.z.max(v2.z);

        Bounds {
            min: Vec3::new(min_x, min_y, min_z),
            max: Vec3::new(max_x, max_y, max_z),
        }
    }

    pub fn surrounding_box(b1: Bounds, b2: Bounds) -> Bounds {
        Bounds {
            min: Vec3 {
                x: b1.min.x.min(b2.min.x),
                y: b1.min.y.min(b2.min.y),
                z: b1.min.z.min(b2.min.z),
            },
            max: Vec3 {
                x: b1.max.x.max(b2.max.x),
                y: b1.max.y.max(b2.max.y),
                z: b1.max.z.max(b2.max.z),
            },
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: Dimension, t_max: Dimension) -> bool {
        Bounds::hit_1d(
            self.min.x,
            self.max.x,
            ray.origin.x,
            ray.direction.x,
            t_min,
            t_max,
        ) && Bounds::hit_1d(
            self.min.y,
            self.max.y,
            ray.origin.y,
            ray.direction.y,
            t_min,
            t_max,
        ) && Bounds::hit_1d(
            self.min.z,
            self.max.z,
            ray.origin.z,
            ray.direction.z,
            t_min,
            t_max,
        )
    }

    fn hit_1d(
        box_min: Dimension,
        box_max: Dimension,
        origin: Dimension,
        direction: Dimension,
        t_min: Dimension,
        t_max: Dimension,
    ) -> bool {
        let inv_d = direction.recip();
        let mut t0 = (box_min - origin) * inv_d;
        let mut t1 = (box_max - origin) * inv_d;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1)
        }
        let ttmin = if t0 > t_min { t0 } else { t_min };
        let ttmax = if t1 < t_max { t1 } else { t_max };

        ttmax > ttmin
    }
}
