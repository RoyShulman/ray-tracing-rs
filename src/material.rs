use crate::{color::Color, hittable::HitRecord, math::Vector3, ray::Ray};

pub struct Scattered {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scattered>;
}

pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let scatter_direction = hit_record.normal + Vector3::random_unit();
        let scatter_direction = if scatter_direction.near_zero() {
            hit_record.normal
        } else {
            scatter_direction
        };

        let scattered = Ray::new(hit_record.p, scatter_direction);
        Some(Scattered {
            ray: scattered,
            attenuation: self.albedo,
        })
    }
}

fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    *v - 2.0 * v.dot(n) * n
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1. { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let reflected = reflect(ray.direction(), &hit_record.normal);
        let reflected = reflected.unit_vector() + self.fuzz * Vector3::random_unit();
        let scattered = Ray::new(hit_record.p, reflected);

        if scattered.direction().dot(&hit_record.normal) <= 0. {
            return None;
        }

        Some(Scattered {
            ray: scattered,
            attenuation: self.albedo,
        })
    }
}
