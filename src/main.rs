#![allow(dead_code)]
pub use nbody::Vec3;
pub use nbody::Particle;
pub use nbody::System;
pub use nbody::greedy;
fn main() {
    let vec1: Vec3 = Vec3::new(1, 1, 1);
    let vec2: Vec3 = Vec3::new(1, 1, 1);
    println!("Here is a vector: {:?}.", vec1);

    let p1: Particle = Particle::new(vec1, vec2, 1);
    let p2: Particle = Particle::new(vec1, vec2, 1);

    let s = System::new(vec![p1, p2]);
    greedy(&s, 1);
}
