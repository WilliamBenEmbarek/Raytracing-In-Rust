use crate::interval::Interval;
use crate::material::Material;
use crate::vec3::{Vec3, dot};
use crate::hittable::{Hittable, HitRecord};
use crate::vec3::Point3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material + Sync + Send>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material + Sync + Send>) -> Self{
        Self { 
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.center - r.origin();
        let a: f64 = r.direction().length_squared();
        let h: f64 = dot(r.direction(), oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        
        let discriminant: f64 = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal: Vec3 = (p - self.center) / self.radius;

        let mut rec: HitRecord = HitRecord {
            p,
            normal: Vec3::new(0.0, 0.0, 0.0), // temp get overwritten by set_face_normal below
            front_face: false, // temp
            t,
            mat: self.material.clone()
        };
        rec.set_face_normal(r, outward_normal);

        return Some(rec);
    }
}