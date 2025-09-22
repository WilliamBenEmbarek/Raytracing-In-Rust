use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::{dot, random_unit_vector, reflect}};
use rand::RngCore;

pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn RngCore) -> Option<Scatter>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self { Self { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn RngCore) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some (Scatter { attenuation: self.albedo, ray: Ray::new(rec.p, scatter_direction) } )
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self { Self { albedo } }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn RngCore) -> Option<Scatter> {
        let reflected = reflect(r_in.direction(), rec.normal);
        if dot(reflected, rec.normal) <= 0.0 {
            return None;
        }
        Some( Scatter {
            attenuation: self.albedo,
            ray: Ray::new(rec.p, reflected)
        })
    }
}