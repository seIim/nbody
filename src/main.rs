#![allow(dead_code)]
pub use nbody::solver::symplectic::SymplecticEuler;
pub use nbody::Vec3;
pub use nbody::Particle;
pub use nbody::System;
pub use nbody::gravity;
fn main() {
    let vec1: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let vec2: Vec3 = Vec3::new(1e5, 1e5, 0.0);
    let vec3: Vec3 = Vec3::new(1e3, 1e3, 0.0);

    let p1: Particle = Particle::new(vec1, Vec3::default(), [Vec3::default(), Vec3::default()], 1e3);
    let p2: Particle = Particle::new(vec2, Vec3::default(), [Vec3::default(), Vec3::default()], 1e30);
    let p3: Particle = Particle::new(vec3, Vec3::default(), [Vec3::default(), Vec3::default()], 1e3);

    let mut s = System::new(vec![p1,p2, p3]);
    
    println!("Before: \n {:?} \n Total Energy: {}", s, s.energy());
    let solver = SymplecticEuler {};
    // s.warmup(SymplecticEuler::warmup);
    println!("Warm up: \n {:?} \n Total Energy: {}", s, s.energy());
    (0..100_000).for_each(|j: i64| {
        if j % 10_000 == 0 { println!("{}", s.energy());}
        s.step(&solver);
    });
    println!("After: \n {:?} \n Total Energy: {}", s, s.energy());
}
