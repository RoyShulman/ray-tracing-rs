use crate::math::Point3;

pub type Color = Point3;

pub trait WriteColor {
    fn write_color(&self) -> String;
}

impl WriteColor for Point3 {
    // This doesn't feel correct, as this is more of a ppm file format, not a geeral point3 format
    fn write_color(&self) -> String {
        let r = (self.x() * 255.999) as u8;
        let g = (self.y() * 255.999) as u8;
        let b = (self.z() * 255.999) as u8;
        format!("{} {} {}", r, g, b)
    }
}
