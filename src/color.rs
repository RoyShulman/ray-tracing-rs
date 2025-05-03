use std::ops::Mul;

use crate::math::Point3;

fn linear_to_gamma(linear_value: f32) -> f32 {
    if linear_value > 0. {
        return linear_value.sqrt();
    }

    0.
}

/// Color is a specific point3
///
/// Useful because we can have specific functions like drawing a color.
/// Also, we keep the color in the range of 0. to 1.0
#[derive(Debug, Default, Clone, Copy)]
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

        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);

        let rbyte = (255.99 * r) as u8;
        let gbyte = (255.99 * g) as u8;
        let bbyte = (255.99 * b) as u8;

        format!("{} {} {}\n", rbyte, gbyte, bbyte)
    }

    pub fn inner(self) -> Point3 {
        self.0
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        // both are colors and have values in the range [0,1] so we keep the invariance
        Color(self.0 * rhs.0)
    }
}
