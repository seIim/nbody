/*
Suite of methods to determine the gravitational force on an object.
TODO: SIMD optional, Barnes-Hut.
*/
pub use crate::System;
pub use crate::Vec3;
pub use crate::Particle;
pub const G: f64 = 6.6743015e-11;

pub struct Force {}

impl Force {
    pub fn greedy (p: &Particle, all_ps: &Vec<Particle>, ind: usize) -> Vec3 {

        let mut others: Vec<_> = Vec::new();
        others.extend_from_slice(&all_ps[..ind]);
        others.extend_from_slice(&all_ps[ind + 1..]);

        let mut f = Vec3::default();
        for _p in others.iter() {
            let d: f64 = (_p.r - p.r).dot(&(_p.r - p.r));
            f = f + (_p.r - p.r)/(d.powf(1.5)) * G * _p.m;
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
            let d: f64 = (_p.r - p.r).dot(&(_p.r - p.r));
            e += G*p.m*_p.m/(d.powf(0.5));
        }
        return e;
    }
}