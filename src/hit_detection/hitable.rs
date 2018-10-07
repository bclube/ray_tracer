use geometry::ray::*;
use geometry::vec3::*;
use world::bounds::*;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: Dimension,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: Dimension, t_max: Dimension) -> Option<HitRecord>;
    fn bounds(&self) -> Option<Bounds>;
}
