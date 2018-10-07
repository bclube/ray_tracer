use geometry::ray::*;
use geometry::vec3::*;
use hit_detection::hitable::*;
use std::rc::*;
use surface::material::*;
use world::bounds::*;

pub struct ModelHitRecord {
    pub hit_record: HitRecord,
    pub material: Rc<Material>,
}

pub trait Model {
    fn hit_model(&self, ray: &Ray, t_min: Dimension, t_max: Dimension) -> Option<ModelHitRecord>;
    fn bounds(&self) -> Option<Bounds>;
}
