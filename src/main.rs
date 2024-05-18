#![allow(dead_code)]
pub use nbody::solver::symplectic::SymplecticEuler;
pub use nbody::Vec3;
pub use nbody::Particle;
pub use nbody::System;
pub use nbody::gravity;
use rand::distributions::{Distribution, Uniform};

fn main() {

    // Example 1: 3 Body problem in 2D
    // let vec1: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    // let vec2: Vec3 = Vec3::new(1e5, 1e5, 0.0);
    // let vec3: Vec3 = Vec3::new(1e3, 1e3, 0.0);

    // let p1: Particle = Particle::new(vec1, Vec3::default(), [Vec3::default(), Vec3::default()], 1e3);
    // let p2: Particle = Particle::new(vec2, Vec3::default(), [Vec3::default(), Vec3::default()], 1e30);
    // let p3: Particle = Particle::new(vec3, Vec3::default(), [Vec3::default(), Vec3::default()], 1e3);

    // let mut s = System::new(vec![p1,p2, p3]);
    
    // println!("Before: \n {:?} \n Total Energy: {}", s, s.energy());
    // let solver = SymplecticEuler {};
    // // s.warmup(SymplecticEuler::warmup);
    // println!("Warm up: \n {:?} \n Total Energy: {}", s, s.energy());
    // (0..100_000).for_each(|j: i64| {
    //     if j % 10_000 == 0 { println!("{}", s.energy());}
    //     s.step(&solver);
    // });
    // println!("After: \n {:?} \n Total Energy: {}", s, s.energy());


    // Example 2: Sampling initial positions from a distribution
    let pos_uniform = Uniform::from(-1e10..1e10);
    let mass_uniform = Uniform::from(0.0..1e30);
    let mut x = rand::thread_rng();
    let mut y = rand::thread_rng();
    let mut z = rand::thread_rng();
    let mut m = rand::thread_rng();
    let mut ps = Vec::<Particle>::new();
    // generate a random sample of 1000 particles and add them to our vec of particles, ps
    for _ in 0..100 {
        ps.push(
            Particle::new(Vec3::new(
                pos_uniform.sample(&mut x),
                pos_uniform.sample(&mut y),
                pos_uniform.sample(&mut z)
            ), Vec3::default(), [Vec3::default(), Vec3::default()],
        mass_uniform.sample(&mut m))
        );
    }
    // create a system of these particles

    let mut s = System::new(ps);

    // we are going to use the symplectic euler method to integrate the system forward in time
    let solver = SymplecticEuler {};
    println!("Total Energy Prior: {}", s.energy());
    (0..100_000).for_each(|j: i64| {
        if j % 10_000 == 0 { println!("{}", s.energy());}
        s.step(&solver);
    });
    println!("Total Energy After: {}", s.energy());


}
