use float_cmp::*;
use geometry::ray::*;
use geometry::vec3::*;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use world::bounds::*;
use world::model::*;

pub struct Tree {
    bounds: Option<Bounds>,
    pub left: Box<Model>,
    pub right: Box<Model>,
}

pub enum SplitDim {
    X,
    Y,
    Z,
}

impl Tree {
    pub fn new(left: Box<Model>, right: Box<Model>) -> Tree {
        let mut bounds: Option<Bounds> = None;
        if let Some(l) = left.bounds() {
            if let Some(r) = right.bounds() {
                bounds = Some(Bounds::surrounding_box(r, l));
            }
        }
        Tree {
            bounds: bounds,
            left: left,
            right: right,
        }
    }

    pub fn from_list(list: &mut Vec<Box<Model>>) -> Box<Model> {
        Tree::from_list_on_dimensions(list, &[SplitDim::X, SplitDim::Y, SplitDim::Z])
    }

    pub fn from_list_on_dimensions(
        list: &mut Vec<Box<Model>>,
        dimensions: &[SplitDim],
    ) -> Box<Model> {
        match list.len() {
            0 => panic!("No models in list: Tree::from_list()"),
            1 => list.remove(0),
            2 => Box::new(Tree::new(list.remove(0), list.remove(0))),
            _ => {
                let idx: HashSet<usize> = dimensions
                    .iter()
                    .flat_map(|ref v| match v {
                        SplitDim::X => vec![0, 1],
                        SplitDim::Y => vec![2, 3],
                        SplitDim::Z => vec![4, 5],
                    })
                    .collect();
                let idx: Vec<_> = idx.iter().collect();
                match thread_rng().choose(idx.as_slice()).unwrap() {
                    0 => list.sort_unstable_by_key(|ref b| FloatCmp(b.bounds().unwrap().min.x)),
                    1 => list.sort_unstable_by_key(|ref b| FloatCmp(b.bounds().unwrap().max.x)),
                    2 => list.sort_unstable_by_key(|ref b| FloatCmp(b.bounds().unwrap().min.y)),
                    3 => list.sort_unstable_by_key(|ref b| FloatCmp(b.bounds().unwrap().max.y)),
                    4 => list.sort_unstable_by_key(|ref b| FloatCmp(b.bounds().unwrap().min.z)),
                    _ => list.sort_unstable_by_key(|ref b| FloatCmp(b.bounds().unwrap().max.z)),
                }
                let half_n = list.len() / 2;
                let mut list_b = list.split_off(half_n);
                Box::new(Tree::new(
                    Tree::from_list_on_dimensions(list, dimensions),
                    Tree::from_list_on_dimensions(&mut list_b, dimensions),
                ))
            }
        }
    }
}

impl Model for Tree {
    fn hit_model(&self, ray: &Ray, t_min: Dimension, t_max: Dimension) -> Option<ModelHitRecord> {
        if let Some(bounds) = self.bounds() {
            if !bounds.hit(ray, t_min, t_max) {
                return None;
            }
        }

        let mut new_max_t = t_max;
        let mut result = None;
        if let Some(hit_left) = self.left.hit_model(ray, t_min, new_max_t) {
            new_max_t = hit_left.hit_record.t;
            result = Some(hit_left);
        }
        if let Some(hit_right) = self.right.hit_model(ray, t_min, new_max_t) {
            result = Some(hit_right);
        }
        result
    }
    fn bounds(&self) -> Option<Bounds> {
        self.bounds
    }
}
