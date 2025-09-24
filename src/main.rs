use rand::Rng;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::{random_vector, Point3, Vec3};
use crate::material::{Dielectric, Lambertian, Material, Metal};
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

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5,0.5, 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0),1000.0, ground_material)));

    let mut rng = rand::rng();


    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.random();
            let center = Point3::new(a as f64 + 0.9 * rng.random::<f64>(), 0.2, b as f64 + 0.9*rng.random::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = random_vector(&mut rng);
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = random_vector(&mut rng);
                    let fuzz = rng.random_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // Glass
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }


    let material_1 = Arc::new(Dielectric::new(1.5));
    let material_2   = Arc::new(Lambertian::new(Color::new(0.4,0.2,0.1)));
    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5),0.0));

    world.add(Box::new(Sphere::new(Point3::new( 0.0,1.0,0.0), 1.0, material_1)));
    world.add(Box::new(Sphere::new(Point3::new(-4.0,1.0,0.0), 1.0, material_2)));
    world.add(Box::new(Sphere::new(Point3::new(4.0,1.0,0.0), 1.0, material_3)));


    let mut camera: Camera = Camera::new()
        .with_aspect_ratio(16.0/9.0)
        .with_image_width(1200)
        .with_samples_per_pixel(500)
        .with_max_depth(50)
        .with_vfov(20.0)
        .with_lookfrom(Point3::new(13.0,2.0,3.0))
        .with_lookat(Point3::new(0.0, 0.0, 0.0))
        .with_vup(Vec3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.6)
        .with_focus_dist(10.0);

    camera.render(&world);
}