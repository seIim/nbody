#![allow(dead_code)]
pub use crate::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub r: Vec3,
    pub v: Vec3,
    pub a: [Vec3; 2], // unused as of right now but will make search much faster later
    pub m: f64
}

impl Particle {
    pub fn new<T: Into<f64>>(r: Vec3, v: Vec3, a: [Vec3; 2], m: T) -> Particle {
        Particle {
            r,
            v,
            a,
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
