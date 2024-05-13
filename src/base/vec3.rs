#![allow(dead_code)]

pub use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Vec3 {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Default for Vec3 {
    fn default() -> Self { Vec3::new(0, 0, 0)}
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Mul<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn mul(self, scalar: T) -> Vec3 {
        let scalar_f64 = scalar.into();
        Vec3 {
            x: self.x * scalar_f64,
            y: self.y * scalar_f64,
            z: self.z * scalar_f64,
        }
    }
}
