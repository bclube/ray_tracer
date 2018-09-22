use geometry::ray::*;
use geometry::vec3::*;

pub struct Camera {
    pub horizontal: Vec3,
    pub lower_left: Vec3,
    pub origin: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov_degrees: Dimension,
        aspect_h_over_v: Dimension,
    ) -> Camera {
        let theta = vfov_degrees * PI_DIMENSION / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_h_over_v * half_height;
        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        Camera {
            horizontal: 2.0 * half_width * u,
            lower_left: look_from - half_width * u - half_height * v - w,
            origin: look_from,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u: Dimension, v: Dimension) -> Ray {
        let horizontal = u * self.horizontal;
        let vertical = v * self.vertical;
        Ray {
            origin: self.origin,
            direction: self.lower_left + horizontal + vertical - self.origin,
        }
    }
}
