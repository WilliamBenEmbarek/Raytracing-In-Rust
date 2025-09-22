use crate::{interval::{self, Interval}, vec3::Vec3};
use std::io::{self, Write};


pub type Color = Vec3;

pub fn color_to_string(pixel_color: Color) -> String {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let intensity = Interval::new(0.000, 0.999);
    let rbyte: u8 = (255.999 * intensity.clamp(r)) as u8;
    let gbyte: u8 = (255.999 * intensity.clamp(g)) as u8;
    let bbyte: u8 = (255.999 * intensity.clamp(b)) as u8;

    format!("{} {} {}\n", rbyte, gbyte, bbyte)
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt()
    }
    return 0.0
}