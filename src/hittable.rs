use std::{ops::RangeInclusive, rc::Rc};

use crate::{
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
    facing: Facing,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vector3, t: f32) -> Self {
        // We assume normal is unit length - is this a good design? probably not

        let facing = match p.dot(&normal) < 0. {
            true => Facing::Front,
            false => Facing::Back,
        };
        let normal = match facing {
            Facing::Back => -normal,
            Facing::Front => normal,
        };

        Self {
            p,
            normal,
            t,
            facing,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, valid_interval: &RangeInclusive<f32>) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    inner: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.inner.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, valid_interval: &RangeInclusive<f32>) -> Option<HitRecord> {
        let mut valid_interval = valid_interval.clone();
        let mut result = None;

        for object in self.inner.iter() {
            let Some(hit_record) = object.hit(ray, &valid_interval) else {
                continue;
            };

            valid_interval = *valid_interval.start()..=hit_record.t;
            result.replace(hit_record);
        }

        result
    }
}
