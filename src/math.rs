use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct VectorD<const D: usize> {
    e: [f32; D],
}

impl<const D: usize> VectorD<D> {
    pub fn length_squared(&self) -> f32 {
        self.e.iter().map(|x| x * x).sum()
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        dot(self, other)
    }
}

fn dot<const D: usize>(u: &VectorD<D>, v: &VectorD<D>) -> f32 {
    u.e.iter().zip(v.e.iter()).map(|(u, v)| u * v).sum()
}

impl<const D: usize> Default for VectorD<D> {
    fn default() -> Self {
        Self { e: [0.; D] }
    }
}

impl<const D: usize> AddAssign for VectorD<D> {
    fn add_assign(&mut self, rhs: Self) {
        for (component, other_component) in self.e.iter_mut().zip(rhs.e.iter()) {
            *component += *other_component;
        }
    }
}

impl<const D: usize> Add for &VectorD<D> {
    type Output = VectorD<D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.e;
        result
            .iter_mut()
            .zip(rhs.e.iter())
            .for_each(|(result, rhs)| *result += *rhs);
        Self::Output { e: result }
    }
}

impl<const D: usize> Add for VectorD<D> {
    type Output = VectorD<D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.e;
        result
            .iter_mut()
            .zip(rhs.e.iter())
            .for_each(|(result, rhs)| *result += *rhs);
        Self::Output { e: result }
    }
}

impl<const D: usize> Add<f32> for VectorD<D> {
    type Output = VectorD<D>;

    fn add(self, rhs: f32) -> Self::Output {
        let mut result = self.e;
        result.iter_mut().for_each(|result| *result += rhs);
        Self::Output { e: result }
    }
}

impl<const D: usize> Sub for VectorD<D> {
    type Output = VectorD<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.e;
        result
            .iter_mut()
            .zip(rhs.e.iter())
            .for_each(|(result, rhs)| *result -= *rhs);
        Self::Output { e: result }
    }
}

impl<const D: usize> Sub for &VectorD<D> {
    type Output = VectorD<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.e;
        result
            .iter_mut()
            .zip(rhs.e.iter())
            .for_each(|(result, rhs)| *result -= *rhs);
        Self::Output { e: result }
    }
}

impl<const D: usize> MulAssign for VectorD<D> {
    fn mul_assign(&mut self, rhs: Self) {
        for (component, other_component) in self.e.iter_mut().zip(rhs.e.iter()) {
            *component *= *other_component;
        }
    }
}

impl<const D: usize> DivAssign for VectorD<D> {
    fn div_assign(&mut self, rhs: Self) {
        for (component, other_component) in self.e.iter_mut().zip(rhs.e.iter()) {
            *component /= *other_component;
        }
    }
}

impl<const D: usize> Div<f32> for &VectorD<D> {
    type Output = VectorD<D>;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = self.e;
        result.iter_mut().for_each(|x| *x /= rhs);
        Self::Output { e: result }
    }
}

impl<const D: usize> Mul<f32> for VectorD<D> {
    type Output = VectorD<D>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self.e;
        result.iter_mut().for_each(|x| *x *= rhs);
        Self::Output { e: result }
    }
}

impl<const D: usize> Mul<VectorD<D>> for f32 {
    type Output = VectorD<D>;

    fn mul(self, rhs: VectorD<D>) -> Self::Output {
        let mut result = rhs.e;
        result.iter_mut().for_each(|x| *x *= self);
        Self::Output { e: result }
    }
}

impl<const D: usize> Mul<&VectorD<D>> for f32 {
    type Output = VectorD<D>;

    fn mul(self, rhs: &VectorD<D>) -> Self::Output {
        let mut result = rhs.e;
        result.iter_mut().for_each(|x| *x *= self);
        Self::Output { e: result }
    }
}

impl<const D: usize> Mul<f32> for &VectorD<D> {
    type Output = VectorD<D>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self.e;
        result.iter_mut().for_each(|x| *x *= rhs);
        Self::Output { e: result }
    }
}

impl VectorD<3> {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { e: [x, y, z] }
    }

    pub const fn x(&self) -> f32 {
        self.e[0]
    }

    pub const fn y(&self) -> f32 {
        self.e[1]
    }

    pub const fn z(&self) -> f32 {
        self.e[2]
    }
}

pub type Point3 = VectorD<3>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let point = Point3::new(1., 2., -1.);
        assert_eq!(point.x(), 1.);
        assert_eq!(point.y(), 2.);
        assert_eq!(point.z(), -1.);
    }

    #[test]
    fn test_default() {
        let point = Point3::default();
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 0.);
        assert_eq!(point.z(), 0.);
    }

    #[test]
    fn test_div_by_scalar() {
        let point = Point3::new(4.0, 2.0, 10.0);
        let result = &point / 2.;

        assert_eq!(result.x(), 2.);
        assert_eq!(result.y(), 1.);
        assert_eq!(result.z(), 5.)
    }

    #[test]
    fn test_div_assign() {
        let point = Point3::new(4.0, 2.0, 10.0);
        let result = &point / 2.;

        assert_eq!(result.x(), 2.);
        assert_eq!(result.y(), 1.);
        assert_eq!(result.z(), 5.)
    }

    #[test]
    fn test_dot() {
        let point = Point3::new(4.0, 2.0, 10.0);
        let result = Point3::new(1.0, 6.0, 10.0);

        assert_eq!(116., point.dot(&result));
    }
}
