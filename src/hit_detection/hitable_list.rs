use geometry::ray::*;
use geometry::vec3::*;
use hit_detection::hitable::*;
use world::bounds::*;

impl<'a> Hitable for &'a [Box<Hitable>] {
    fn hit(&self, ray: &Ray, t_min: Dimension, t_max: Dimension) -> Option<HitRecord> {
        let mut closest_t = t_max;
        self.iter().fold(None, |acc: Option<HitRecord>, h| {
            if let Some(hit) = h.hit(ray, t_min, closest_t) {
                closest_t = hit.t;
                Some(hit)
            } else {
                acc
            }
        })
    }

    fn bounds(&self) -> Option<Bounds> {
        None
    }
}
