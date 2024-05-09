/*
Suite of methods to determine the gravitational force on an object.
This may turn into multiple modules.
TODO: Greedy Algorithm, SIMD optional, Barnes-Hut.
*/
use crate::System;
pub use crate::Vec3;

// Greedy takes as input the system and the value of the particle in the system array.
pub fn greedy (sys: &System, n: usize) {
    println!("here u go {:?}", sys.particles[n]);
}