use geometry::ray::*;
use geometry::vec3::*;
use hit_detection::hitable::*;
use std::rc::*;
use surface::material::*;
use world::bounds::*;
use world::model::*;

pub struct WorldEntity {
    pub shape: Box<Hitable>,
    pub material: Rc<Material>,
}

impl Model for WorldEntity {
    fn hit_model(&self, ray: &Ray, t_min: Dimension, t_max: Dimension) -> Option<ModelHitRecord> {
        if let Some(hit) = self.shape.hit(ray, t_min, t_max) {
            Some(ModelHitRecord {
                hit_record: hit,
                material: self.material.clone(),
            })
        } else {
            None
        }
    }

    fn bounds(&self) -> Option<Bounds> {
        self.shape.bounds()
    }
}
