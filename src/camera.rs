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
    pixel_samples_scale: f32,
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
            pixel_samples_scale: 1. / samples_per_pixel as f32,
        }
    }

    pub fn render_ppm(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for height in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", (self.image_height - height));
            for width in 0..self.image_width {
                let mut pixel_color = Point3::default();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(width, height);
                    pixel_color += ray_color(&ray, world, 0).inner();
                }
                // now we need to scale the color by the number of samples
                let pixel_color = Color::new(
                    pixel_color.x() * self.pixel_samples_scale,
                    pixel_color.y() * self.pixel_samples_scale,
                    pixel_color.z() * self.pixel_samples_scale,
                );

                println!("{}", pixel_color.write_ppm())
            }
        }

        eprintln!("Done             ");
    }

    fn get_ray(&self, width: u16, height: u16) -> Ray {
        // get a random point in the [-0.5, -0.5]-[0.5, 0.5] square
        let offset_x = rand::random_range(-0.5..=0.5);
        let offset_y = rand::random_range(-0.5..=0.5);

        let pixel_center = self.pixel00_loc
            + ((width as f32 + offset_x) * self.pixel_delta_u)
            + ((height as f32 + offset_y) * self.pixel_delta_v);

        let direction = pixel_center - self.center;
        Ray::new(self.center, direction)
    }
}

const MAX_DEPTH: usize = 10;

fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Color {
    if depth > MAX_DEPTH {
        return Color::default();
    }

    if let Some(t) = world.hit(ray, &(0.001..=f32::INFINITY)) {
        let direction = t.normal + Vector3::random_in_hemisphere(&t.normal);
        // bounce the ray by creating a new one from the hit point in the direction
        let new_ray = Ray::new(t.p, direction);
        // on each bounce, reduce the color by half
        let color = ray_color(&new_ray, world, depth + 1);
        let half_color = color.inner() * 0.5;
        // we know the color is in the range [0, 1]
        return Color::new(half_color.x(), half_color.y(), half_color.z());
    }

    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.);
    let color = (1. - a) * Point3::new(1., 1., 1.) + a * Point3::new(0.5, 0.7, 1.);
    Color::new(color.x(), color.y(), color.z())
}
