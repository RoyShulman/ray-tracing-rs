use std::ops::{Add, AddAssign, Mul};

use crate::math::Point3;

/// Color is a specific point3
///
/// Useful because we can have specific functions like default
/// or avoid multiplying a color by a color which makes no sense
#[derive(Debug, Default)]
pub struct Color(Point3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        debug_assert!((0. ..=1.).contains(&r), "r must be between 0 and 1");
        debug_assert!((0. ..=1.).contains(&g), "g must be between 0 and 1");
        debug_assert!((0. ..=1.).contains(&b), "b must be between 0 and 1");
        Self(Point3::new(r, g, b))
    }

    pub fn write_ppm(&self) -> String {
        let r = self.0.x();
        let g = self.0.y();
        let b = self.0.z();

        let rbyte = (255.99 * r) as u8;
        let gbyte = (255.99 * g) as u8;
        let bbyte = (255.99 * b) as u8;

        format!("{} {} {}\n", rbyte, gbyte, bbyte)
    }

    pub fn inner(self) -> Point3 {
        self.0
    }
}
