#![allow(dead_code)]
pub use nbody::Vec3;
pub use nbody::Particle;
pub use nbody::System;
pub use nbody::gravity;
pub use nbody::symplectic::Solver;
fn main() {
    let vec1: Vec3 = Vec3::new(20.0, 20.0, 0.0);
    let vec2: Vec3 = Vec3::new(1e5, 1e5, 0.0);
    let vec3: Vec3 = Vec3::new(1e7, 1e3, 0.0);

    let p1: Particle = Particle::new(vec1, Vec3::default(), [Vec3::default(), Vec3::default()], 1e31);
    let p2: Particle = Particle::new(vec2, Vec3::default(), [Vec3::default(), Vec3::default()], 1e15);
    let p3: Particle = Particle::new(vec3, Vec3::default(), [Vec3::default(), Vec3::default()], 1e10);

    let mut s = System::new(vec![p1, p2, p3]);
    println!("Before: {:?}", s.energy());
    // for whatever you need a 1 iteration warmup before the energy is roughly conserved
    // this makes sense because 0 velocity in the system is non physical
    // with most nbody systems we can estimate initial velocity so this would not be a problem
    (0..1).for_each(|_: i64| {
        s.step(Solver::verlet_first_order);
    });
    println!("Before: {:?}", s.energy());
    (0..100_000).for_each(|_: i64| {
        s.step(Solver::verlet_first_order);
    });
    println!("After: {:?}", s.energy());
    println!("After: {:?}", s);
}
