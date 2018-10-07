use geometry::ray::*;
use geometry::vec3::*;
use hit_detection::hitable::*;
use std::sync::Arc;
use surface::material::*;
use world::bounds::*;

pub struct ModelHitRecord {
    pub hit_record: HitRecord,
    pub material: Arc<Material>,
}

pub trait Model {
    fn hit_model(&self, ray: &Ray, t_min: Dimension, t_max: Dimension) -> Option<ModelHitRecord>;
    fn bounds(&self) -> Option<Bounds>;
}

pub type ModelSS = Model + Sync + Send;
