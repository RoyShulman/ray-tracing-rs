use camera::Camera;
use hittable::HittableList;
use math::Point3;
use sphere::Sphere;

mod camera;
mod color;
mod hittable;
mod math;
mod ray;
mod sphere;

const IMAGE_WIDTH: u16 = 400;
const ASPECT_RATIO: f32 = 16. / 9.;
const SAMPLES_PER_PIXEL: u8 = 10;

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, SAMPLES_PER_PIXEL);
    camera.render_ppm(&world);
}
