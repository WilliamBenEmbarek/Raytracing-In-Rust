use crate::interval::Interval;
use crate::material::Material;
use crate::vec3::{Point3, Vec3, dot};
use crate::ray::Ray;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: Arc<dyn Material + Send + Sync>
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        if self.front_face { 
            self.normal = outward_normal; 
        } else { 
            self.normal = -outward_normal; 
        };    
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}