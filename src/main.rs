use std::rc::Rc;

use hittable::{Hittable, HittableList};
use math::Point3;
use ray::Ray;

use color::{Color, WriteColor};
use sphere::Sphere;

mod color;
mod hittable;
mod math;
mod ray;
mod sphere;

const IMAGE_WIDTH: u16 = 400;
const ASPECT_RATIO: f32 = 16. / 9.;

fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
    if let Some(hit_point) = world.hit(ray, &(0.0..=f32::INFINITY)) {
        return 0.5 * (hit_point.normal + 1.);
    };

    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.);
    (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
}

fn main() {
    let image_height = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;
    let image_height = match image_height < 1 {
        true => 1,
        false => image_height,
    };

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f32 / image_height as f32);
    let camera_center = Point3::default();

    let viewport_u = Point3::new(viewport_width, 0., 0.);
    let viewport_v = Point3::new(0., -viewport_height, 0.);

    let pixel_delta_u = &viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = &viewport_v / image_height as f32;

    let viewport_upper_left =
        &camera_center - &Point3::new(0., 0., focal_length) - &viewport_u / 2. - &viewport_v / 2.;
    let pixel00_loc = &viewport_upper_left + &(0.5 * (&pixel_delta_u + &pixel_delta_v));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    draw_image(
        image_height,
        IMAGE_WIDTH,
        &pixel00_loc,
        &pixel_delta_u,
        &pixel_delta_v,
        &camera_center,
        &world,
    );
}

fn draw_image<T: Hittable>(
    image_height: u16,
    image_width: u16,
    pixel00_loc: &Point3,
    pixel_delta_u: &Point3,
    pixel_delta_v: &Point3,
    camera_center: &Point3,
    world: &T,
) {
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for height in 0..image_height {
        eprintln!("Scanlines remaining: {}", (image_height - height));
        for width in 0..image_width {
            let pixel_center =
                pixel00_loc + &(&(width as f32 * pixel_delta_u) + &(height as f32 * pixel_delta_v));
            let ray_direction = &pixel_center - camera_center;

            let ray = Ray::new(camera_center.clone(), ray_direction);

            let color = ray_color(&ray, world);

            println!("{}", color.write_color())
        }
    }

    eprintln!("Done             ");
}
