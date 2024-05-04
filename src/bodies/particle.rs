pub use crate::base::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    r: Vec3,
    v: Vec3,
    m: f64
}

impl Particle {
    pub fn new<T: Into<f64>>(r: Vec3, v: Vec3, m: T) -> Particle {
        Particle {
            r,
            v,
            m: m.into(),
        }
    }

    pub fn get_t(self) -> f64 {
        return 0.5*(self.m)*(self.v.dot(&self.v));
    }
}

fn main () {
    println!("Hello world!");
}