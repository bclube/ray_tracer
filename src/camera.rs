use geometry::ray::*;
use geometry::vec3::*;

pub struct Camera {
    pub horizontal: Vec3,
    pub lower_left: Vec3,
    pub origin: Vec3,
    pub vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: Dimension,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov_degrees: Dimension,
        aspect_h_over_v: Dimension,
        aperture: Dimension,
        focus_dist: Dimension,
    ) -> Camera {
        let theta = vfov_degrees * PI_DIMENSION / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_h_over_v * half_height;
        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        Camera {
            horizontal: 2.0 * focus_dist * half_width * u,
            lower_left: look_from
                - focus_dist * half_width * u
                - focus_dist * half_height * v
                - focus_dist * w,
            origin: look_from,
            vertical: 2.0 * focus_dist * half_height * v,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: Dimension, t: Dimension) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
