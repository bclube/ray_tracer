use geometry::vec3::*;

pub struct Ray {
    pub direction: Vec3,
    pub origin: Vec3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: Dimension) -> Vec3 {
        self.origin + self.direction * t
    }
}
