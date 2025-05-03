use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable::HittableList;
use material::{Lambertian, Metal};
use math::Point3;
use sphere::Sphere;

mod camera;
mod color;
mod hittable;
mod material;
mod math;
mod ray;
mod sphere;

const IMAGE_WIDTH: u16 = 400;
const ASPECT_RATIO: f32 = 16. / 9.;
const SAMPLES_PER_PIXEL: u8 = 100;

fn main() {
    let ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.2),
        0.5,
        center,
    )));
    world.add(Box::new(Sphere::new(Point3::new(-1., 0., -1.), 0.5, left)));
    world.add(Box::new(Sphere::new(Point3::new(1., 0., -1.), 0.5, right)));

    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, SAMPLES_PER_PIXEL);
    camera.render_ppm(&world);
}
