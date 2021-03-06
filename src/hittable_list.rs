use super::Ray;
use super::{HitRecord, Hittable};

use std::{
    boxed::Box,
    marker::{Send, Sync},
};

#[derive(Debug, Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn add(&mut self, obj: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(obj)
    }

    pub fn chain_add(mut self, obj: Box<dyn Hittable + Sync + Send>) -> Self {
        self.add(obj);
        self
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Find the object that hit the ray first
        self.objects.iter().fold(None, |best, obj| {
            let closest_so_far = best.as_ref().map_or(t_max, |x| x.t);
            obj.hit(ray, t_min, closest_so_far).or(best)
        })
    }
}
