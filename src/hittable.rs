use std::{ops::RangeInclusive, rc::Rc};

use crate::{
    material::Material,
    math::{Point3, Vector3},
    ray::Ray,
};

pub enum Facing {
    Front,
    Back,
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f32,
    pub mat: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(ray: &Ray, outward_normal: Vector3, t: f32, mat: Rc<dyn Material>) -> Self {
        let p = ray.at(t);

        let facing = match ray.direction().dot(&outward_normal) < 0. {
            true => Facing::Front,
            false => Facing::Back,
        };
        let normal = match facing {
            Facing::Front => outward_normal,
            Facing::Back => -outward_normal,
        };

        Self { p, normal, t, mat }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, acceptable_range: &RangeInclusive<f32>) -> Option<HitRecord>;
}

pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittables: Vec::new(),
        }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }

    pub fn hit(&self, ray: &Ray, acceptable_range: &RangeInclusive<f32>) -> Option<HitRecord> {
        let mut closest_so_far = *acceptable_range.end();
        let mut closest_hit = None;

        for hittable in &self.hittables {
            if let Some(hit) = hittable.hit(ray, acceptable_range) {
                if hit.t < closest_so_far {
                    closest_so_far = hit.t;
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }
}
