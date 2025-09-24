use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign};
use rand::Rng;
use rayon::vec;

use crate::vec3;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vec3 {
    pub e: [f64; 3],
}

// Optional alias (matches the C++ point3 = vec3)
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 { 
        self.e[0] 
    }
    pub fn y(&self) -> f64 { 
        self.e[1] 
    }
    pub fn z(&self) -> f64 { 
        self.e[2] 
    }

    pub fn length_squared(&self) -> f64 { 
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2] 
    }
    pub fn length(&self) -> f64 { 
        self.length_squared().sqrt()
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self::Output{
        Self::new(
            self.e[0] * t,
            self.e[1] * t,
            self.e[2] * t,
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Div<f64> for Vec3{
    type Output = Self;
    fn div(self, t: f64) -> Self::Output {
        Self::new(
            self.e[0] / t,
            self.e[1] / t,
            self.e[2] / t,
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(
            -self.e[0],
            -self.e[1],
            -self.e[2],
        )
    }

}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] +
    u.e[1] * v.e[1] +
    u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0]
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_disk(rng: &mut dyn rand::RngCore) -> Vec3 {
    loop {
        let p: Vec3 = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

pub fn random_unit_vector(rng: &mut dyn rand::RngCore) -> Vec3 {
    loop {
        let p: Vec3 = random_with_range(-1.0, 1.0, rng);
        let lensq: f64 = p.length_squared();
        if (lensq <= 1.0 && lensq > 1e-160) {
            return p / lensq.sqrt()
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3, rng: &mut dyn rand::RngCore) -> Vec3 {
    let on_unit_sphere = random_unit_vector(rng);
    if (dot(on_unit_sphere, normal) > 0.0) {
        return on_unit_sphere
    } else {
        return -on_unit_sphere
    }
}

pub fn random_vector(rng: &mut dyn rand::RngCore) -> Vec3{
    Vec3::new(
        rng.random_range(0.0..1.0), rng.random_range(0.0..1.0), rng.random_range(0.0..1.0)
    )
}

pub fn random_with_range(min: f64, max: f64, rng: &mut dyn rand::RngCore) -> Vec3 {
    Vec3::new(rng.random_range(min..max), rng.random_range(min..max), rng.random_range(min..max))
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3{
    v - (normal * dot(v,normal) * 2.0)
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = (uv + (n * cos_theta)) * etai_over_etat;
    let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}