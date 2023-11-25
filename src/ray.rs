use crate::math::Point3;

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3 {
        &self.origin + &(&self.direction * t)
    }

    pub fn direction(&self) -> &Point3 {
        &self.direction
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let origin = Point3::new(1., 1., 1.);
        let direction = Point3::new(2., -1., 5.);
        let ray = Ray::new(origin, direction);
        let expected = Point3::new(2., 0.5, 3.5);

        assert_eq!(expected, ray.at(0.5));
    }
}
