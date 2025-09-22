use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use crate::material::{Dielectric, Lambertian, Metal};
use std::sync::Arc;

mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;
mod material;


fn main() {
    // STAR PLATINUM THE WORLD! BWOOOOOSH
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8,0.8,0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1,0.2,0.5)));
    let material_left   = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right  = Arc::new(Metal::new(Color::new(0.8,0.6,0.2), 0.001));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0),100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,0.0,-1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,0.0,-1.0), 0.4, material_bubble)));
    world.add(Box::new(Sphere::new(Point3::new(1.0,0.0,-1.0), 0.5, material_right)));


    let mut camera: Camera = Camera::new().with_aspect_ratio(16.0/9.0).with_image_width(1600).with_vfov(90.0);
    camera.render(&world);
}