#![allow(dead_code)]
pub use crate::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub r: Vec3,
    pub v: Vec3,
    pub m: f64
}

impl Particle {
    pub fn new<T: Into<f64>>(r: Vec3, v: Vec3, m: T) -> Particle {
        Particle {
            r,
            v,
            m: m.into(),
        }
    }

    pub fn update_r(&mut self, value: Vec3) {
        self.r = value;
    }

    pub fn update_v(&mut self, value: Vec3) {
        self.v = value;
    }

    pub fn update_m(&mut self, value: f64) {
        self.m = value;
    }
}
