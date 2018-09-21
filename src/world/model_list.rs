use geometry::ray::*;
use geometry::vec3::*;
use world::model::*;

impl<'a> Model for &'a [Box<Model>] {
    fn hit_model(&self, ray: &Ray, t_min: Dimension, t_max: Dimension) -> Option<ModelHitRecord> {
        let mut closest_t = t_max;
        let mut closest_hit: Option<ModelHitRecord> = None;
        for model in self.iter() {
            if let Some(hit) = model.hit_model(ray, t_min, closest_t) {
                let t = hit.hit_record.t;
                if t < closest_t {
                    closest_t = t;
                    closest_hit = Some(hit);
                }
            }
        }
        closest_hit
    }
}
