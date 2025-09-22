use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3}};
use rand::{Rng, RngCore};

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
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self { Self { albedo, fuzz } }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn RngCore) -> Option<Scatter> {
        let reflected = unit_vector(reflect(r_in.direction(), rec.normal)) + (random_unit_vector(rng) * self.fuzz);
        if dot(reflected, rec.normal) <= 0.0 {
            return None;
        }
        Some( Scatter {
            attenuation: self.albedo,
            ray: Ray::new(rec.p, reflected)
        })
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self { Self {refraction_index} }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn RngCore) -> Option<Scatter> {
        let ri: f64 = if rec.front_face {
            1.0/self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = unit_vector(r_in.direction());

        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction: Vec3 = if (cannot_refract || reflectance(cos_theta, ri) > rng.random()) {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, ri)
        };
        Some( Scatter {
            attenuation: Color::new(1.0,1.0,1.0),
            ray: Ray::new(rec.p, direction)
        })
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Using schlicks approximation
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0-cosine).powi(5)
}
