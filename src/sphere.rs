use std::ops::RangeInclusive;

use crate::{
    hittable::{HitRecord, Hittable},
    math::Point3,
    ray::Ray,
};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, valid_interval: &RangeInclusive<f32>) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();

        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let Some(root) = find_root_in_range(half_b, a, sqrtd, valid_interval) else {
            return None;
        };

        let p = ray.at(root);
        let normal = (&p - &self.center) / self.radius;

        Some(HitRecord::new(p, normal, root))
    }
}

fn find_root_in_range(
    half_b: f32,
    a: f32,
    sqrtd: f32,
    valid_interval: &RangeInclusive<f32>,
) -> Option<f32> {
    let first_root = (-half_b - sqrtd) / a;
    if valid_interval.contains(&first_root) {
        return Some(first_root);
    }

    let second_root = (-half_b + sqrtd) / a;
    if valid_interval.contains(&second_root) {
        return Some(second_root);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_sphere() {
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5);
        let ray = Ray::new(
            Point3::new(0., 0., 0.),
            Point3::new(-0.2888888, -0.48888892, -1.0),
        );

        let hit_point = sphere.hit(&ray, &(-1.0..=1.0)).unwrap();
        assert_eq!(hit_point.t, 0.68790466);

        let hit_point_outside_interval = sphere.hit(&ray, &(100.0..=101.0));
        assert!(hit_point_outside_interval.is_none());
    }

    #[test]
    fn test_no_hit_sphere() {
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5);
        let ray = Ray::new(Point3::new(5.5, 0., 0.), Point3::new(0., 0., -1.0));

        assert!(sphere.hit(&ray, &(-1000.0..=-1000.0)).is_none());
    }
}
