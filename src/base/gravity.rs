/*
Suite of methods to determine the gravitational force on an object.
This may turn into multiple modules.
TODO: Greedy Algorithm, SIMD optional, Barnes-Hut.
*/
pub use crate::System;
pub use crate::Vec3;
pub use crate::Particle;
pub const G: f64 = 6.674e-11;

pub fn distance(p_1: &Vec3, p_2: &Vec3) -> f64 {
    ((p_1.x-p_2.x).powf(2.0)+ (p_1.y-p_2.y).powf(2.0) + (p_1.z-p_2.z).powf(2.0)).powf(0.5)
}

// Greedy takes as input the system and the value of the particle in the system array.
pub fn greedy (p: &Particle, others: &Vec<Particle>) -> f64 {
    let mut force: f64 = 0.0;
    for _p in others {
        force += G*p.m*_p.m/distance(&p.r, &_p.r);
    }
    return force;
}