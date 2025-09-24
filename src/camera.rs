use rand::Rng;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::{color::{color_to_string, Color}, hittable::Hittable, interval::Interval, ray::Ray, vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3}};


static TRANS_FLAG: bool = false;

pub struct Camera {
    pub  aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    // Camera location / FOV settings
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3, // Camera-relative "up"

    pub defocus_angle: f64,
    pub focus_dist: f64,

    // Camera frame basis vectors
    u: Vec3, 
    v: Vec3,
    w: Vec3,

    // Private fields that get initialized later
    image_height: Option<i32>,
    pixel_samples_scale: Option<f64>,
    center: Option<Point3>,
    pixel00_loc: Option<Point3>,
    pixel_delta_u: Option<Vec3>,
    pixel_delta_v: Option<Vec3>,
    defocus_disk_u: Option<Vec3>,
    defocus_disk_v: Option<Vec3>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 500,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0,0.0,-1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            u: Vec3::new(0.0, 0.0, 0.0), //Blank vectors to begin with, should they be options? dunno maybe
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            image_height: None,
            pixel_samples_scale: None,
            center: None,
            pixel00_loc: None,
            pixel_delta_u: None,
            pixel_delta_v: None,
            defocus_disk_u: None,
            defocus_disk_v: None,
        }
    }
    
    pub fn with_aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_max_depth(mut self, max_depth: i32) -> Self {
        self.max_depth = max_depth;
        self
    }
    
    pub fn with_samples_per_pixel(mut self, samples_per_pixel: i32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn with_image_width(mut self, image_width: i32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn with_vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn with_lookfrom(mut self, lookfrom: Point3) -> Self {
        self.lookfrom = lookfrom;
        self
    }

    pub fn with_lookat(mut self, lookat: Point3) -> Self {
        self.lookat = lookat;
        self
    }

    pub fn with_vup(mut self, vup: Point3) -> Self {
        self.vup = vup;
        self
    }

    pub fn with_defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn with_focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    // Setters for updating after creation
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
        // Invalidate computed values so they get recalculated
        self.image_height = None;
        self.pixel_delta_u = None;
        self.pixel_delta_v = None;
        self.pixel00_loc = None;
    }

    pub fn set_image_width(&mut self, image_width: i32) {
        self.image_width = image_width;
        // Invalidate computed values
        self.image_height = None;
        self.pixel_delta_u = None;
        self.pixel_delta_v = None;
        self.pixel00_loc = None;
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        
        let image_height = self.image_height.unwrap();
        let pixel00_loc = self.pixel00_loc.unwrap();
        let pixel_delta_u = self.pixel_delta_u.unwrap();
        let pixel_delta_v = self.pixel_delta_v.unwrap();
        let center = self.center.unwrap();

        // Extract all needed data from self BEFORE the parallel operation
        let image_width = self.image_width;
        let samples_per_pixel = self.samples_per_pixel;
        let pixel_samples_scale = self.pixel_samples_scale.unwrap();

        println!("P3\n{} {}\n255", self.image_width, image_height);

        let completed_rows = AtomicUsize::new(0);

        let rows: Vec<Vec<String>> = (0..image_height)
            .into_par_iter()
            .map(|j| {
                let mut row_pixels = Vec::new();

                for i in 0..self.image_width {
                    let mut rng = rand::rng();
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..self.samples_per_pixel {
                        let r: Ray = self.get_ray(i, j, &mut rng);
                        pixel_color += self.ray_color(r, self.max_depth, world, &mut rng);
                    }

                    row_pixels.push(color_to_string(pixel_color * self.pixel_samples_scale.unwrap()));
                }
                let completed = completed_rows.fetch_add(1, Ordering::Relaxed) + 1;
                eprintln!("\rScanlines remaining: {} ", image_height as usize - completed);

                row_pixels
            })
            .collect();
        // Output all rows in order
        for row in rows {
            for pixel_str in row {
                print!("{}", pixel_str);
            }
        }

        eprintln!("\rDone.");     
    }

    fn initialize(&mut self) {
        self.image_height = Some((self.image_width as f64 / self.aspect_ratio) as i32);
        self.image_height = Some(self.image_height.unwrap().max(1));
        self.pixel_samples_scale = Some(1.0 / self.samples_per_pixel as f64);

        self.center = Some(self.lookfrom);

        // Determine viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height.unwrap() as f64);

        // Calculate u,v,w unit basis vectors for camera coordinate frame.
        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w,self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u =  self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = Some(viewport_u / self.image_width as f64);
        self.pixel_delta_v = Some(viewport_v / self.image_height.unwrap() as f64);

        // Calculcate location of upper left pixel
        let viewport_upper_left = self.center.unwrap() - (self.w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = Some(viewport_upper_left + (self.pixel_delta_u.unwrap() + self.pixel_delta_v.unwrap()) * 0.5);
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();

        self.defocus_disk_u = Some(self.u * defocus_radius);
        self.defocus_disk_v = Some(self.v * defocus_radius);
        
    
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable, rng: &mut impl rand::RngCore) -> Color {
        if depth <= 0 {
            return Color::new(0.0,0.0,0.0)
        }
        if let Some(rec) = world.hit(&r, Interval::new(0.001, f64::INFINITY)) { 
            if let Some(sc) = rec.mat.scatter(&r, &rec, rng) {
                return sc.attenuation * self.ray_color(sc.ray, depth - 1, world, rng);        
            }
            return Color::new(0.0,0.0,0.0) 
        }

        if TRANS_FLAG {
            let a = 0.5 * (unit_vector(r.direction()).y() + 1.0); // 0 at bottom, 1 at top
            let c_blue  = Color::new(0.357, 0.808, 0.980); // #5BCEFA
            let c_pink  = Color::new(0.961, 0.663, 0.722); // #F5A9B8
            let c_white = Color::new(1.0,  1.0,   1.0);

            if a >= 0.8 {
                return c_blue;   // top 20%
            } else if a >= 0.6 {
                return c_pink;   // next 20%
            } else if a >= 0.4 {
                return c_white;  // middle 20%
            } else if a >= 0.2 {
                return c_pink;   // next 20%
            } else {
                return c_blue;   // bottom 20%
            }
        } else {
            let unit_direction = unit_vector(r.direction());
            let a = 0.5 * (unit_direction.y() + 1.0);
            (Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Color::new(0.5, 0.7, 1.0) * a)
        }
    }

    fn get_ray(&self, i: i32, j: i32, rng: &mut impl rand::RngCore) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly sampled point around the pixel location i,j
        let offset = self.sample_square(rng);
        let pixel_sample = self.pixel00_loc.unwrap()
                            + (self.pixel_delta_u.unwrap() * (i as f64 + offset.x()))
                            + (self.pixel_delta_v.unwrap() * (j as f64 + offset.y()));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center.unwrap()
        } else {
            self.defocus_disk_sample(rng)
        };
        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    fn sample_square(&self, rng: &mut impl rand::RngCore) -> Vec3 {
        Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self, rng: &mut impl rand::RngCore) -> Point3 {
        let p = random_in_unit_disk(rng);
        self.center.unwrap() + (self.defocus_disk_u.unwrap() * p.x()) + (self.defocus_disk_v.unwrap() * p.y())
    }
}

