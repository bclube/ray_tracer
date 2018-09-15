use geometry::ray::*;
use geometry::vec3::*;

pub struct Camera {
    pub horizontal: Vec3,
    pub lower_left: Vec3,
    pub origin: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: Dimension, v: Dimension) -> Ray {
        let horizontal = u * self.horizontal;
        let vertical = v * self.vertical;
        Ray {
            origin: self.origin,
            direction: self.lower_left + horizontal + vertical - self.origin,
        }
    }
}
