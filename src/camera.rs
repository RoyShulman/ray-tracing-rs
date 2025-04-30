use rand::Rng;

use crate::{
    color::Color,
    hittable::HittableList,
    math::{Point3, Vector3},
    ray::Ray,
};

fn get_image_height(image_width: u16, aspect_ratio: f32) -> u16 {
    let image_height = (image_width as f32 / aspect_ratio) as u16;
    image_height.max(1)
}

pub struct Camera {
    image_height: u16,
    image_width: u16,
    samples_per_pixel: u8,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(image_width: u16, aspect_ratio: f32, samples_per_pixel: u8) -> Self {
        let image_height = get_image_height(image_width, aspect_ratio);

        let focal_length = 1.;
        let viewport_height = 2.;
        // use the actual ratio instead of `aspect_ratio` as that one is the ideal one
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let center = Point3::default();

        let viewport_u = Point3::new(viewport_width, 0., 0.);
        let viewport_v = Point3::new(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // start from the center. Move focal length to the z axis, then move left half of the viewport
        // and up half of the viewport
        let viewport_upper_left =
            center - Point3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        // nudge the ray to hit the center of the pixel instead of the top left corner
        let pixel00_loc = viewport_upper_left + (0.5 * (pixel_delta_u + pixel_delta_v));

        Self {
            image_height,
            image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }

    pub fn render_ppm(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for height in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", (self.image_height - height));
            for width in 0..self.image_width {
                // for _ in 0..self.samples_per_pixel {
                let ray = self.get_ray(width, height);
                // pixel_color += ray_color(&ray, world);
                // }

                let pixel_color = ray_color(&ray, world);
                println!("{}", pixel_color.write_ppm(self.samples_per_pixel))
            }
        }

        eprintln!("Done             ");
    }

    fn get_ray(&self, width: u16, height: u16) -> Ray {
        let pixel_center = self.pixel00_loc
            + (width as f32 * self.pixel_delta_u)
            + (height as f32 * self.pixel_delta_v);

        let direction = pixel_center - self.center;
        Ray::new(self.center, direction)
    }

    fn pixel_sample_square(&self) -> Vector3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen::<f32>();
        let py = -0.5 + rng.gen::<f32>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(t) = world.hit(ray, &(0.0..=f32::INFINITY)) {
        // we want to map the normal to a color
        let normal_color = 0.5 * (t.normal + Point3::new(1., 1., 1.));
        return Color::new(normal_color.x(), normal_color.y(), normal_color.z());
    }

    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.);
    (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
}
