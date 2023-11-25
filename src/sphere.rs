use crate::{math::Point3, ray::Ray};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn hit_sphere(&self, ray: &Ray) -> Option<f32> {
        let a = ray.direction().length_squared();
        let oc = ray.origin() - &self.center;
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            None
        } else {
            let hit_point = (-half_b - discriminant.sqrt()) / a;
            Some(hit_point)
        }
    }
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

        let hit_point = sphere.hit_sphere(&ray).unwrap();
        assert_eq!(hit_point, 0.68790466);
    }

    #[test]
    fn test_no_hit_sphere() {
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5);
        let ray = Ray::new(Point3::new(5.5, 0., 0.), Point3::new(0., 0., -1.0));

        assert!(sphere.hit_sphere(&ray).is_none());
    }
}
