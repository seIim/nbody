/*
Suite of methods to determine the gravitational force on an object.
This may turn into multiple modules.
TODO: SIMD optional, Barnes-Hut.
*/
pub use crate::System;
pub use crate::Vec3;
pub use crate::Particle;
pub const G: f64 = 6.674e-11;

pub fn distance(p_1: &Vec3, p_2: &Vec3) -> f64 {
    ((p_1.x-p_2.x).powf(2.0)+ (p_1.y-p_2.y).powf(2.0) + (p_1.z-p_2.z).powf(2.0)).powf(0.5)
}

pub struct Force {}

impl Force {
    pub fn greedy (p: &Particle, all_ps: &Vec<Particle>, ind: usize) -> f64 {
        let particles_clone = all_ps.clone(); // Clone the particles vector

        let mut others: Vec<_> = Vec::new();
        others.extend_from_slice(&particles_clone[..ind]);
        others.extend_from_slice(&particles_clone[ind + 1..]);

        let mut f: f64 = 0.0;
        for _p in others {
            f += G*_p.m/(distance(&p.r, &_p.r).powi(2));
        }
        return f;
    }
}
pub struct Energy {}

impl Energy {
    pub fn greedy (p: &Particle, all_ps: &Vec<Particle>, ind: usize) -> f64 {
    
        let mut others: Vec<_> = Vec::new();
        others.extend_from_slice(&all_ps[..ind]);
        others.extend_from_slice(&all_ps[ind + 1..]);
    
        let mut e: f64 = 0.0;
        for _p in others {
            // potential energy
            e += G*_p.m/distance(&p.r, &_p.r);
        }
        return e;
    }
}