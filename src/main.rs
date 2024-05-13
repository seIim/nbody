#![allow(dead_code)]
pub use nbody::Vec3;
pub use nbody::Particle;
pub use nbody::System;
pub use nbody::gravity;
pub use nbody::symplectic::Solver;
fn main() {
    let vec1: Vec3 = Vec3::new(0, 0, 0);
    let vec2: Vec3 = Vec3::new(10e5, 10e5, 10e5);
    let vec3: Vec3 = Vec3::new(5e5, 5e5, 5e5);

    let p1: Particle = Particle::new(vec1, Vec3::default(), 1e10);
    let p2: Particle = Particle::new(vec2, Vec3::default(), 1e7);
    let p3: Particle = Particle::new(vec3, Vec3::default(), 1e8);


    let mut s = System::new(vec![p1, p2, p3]);
    println!("Before: {:?}", s);
    s.step(Solver::verlet_first_order);
    println!("After: {:?}", s);
}
