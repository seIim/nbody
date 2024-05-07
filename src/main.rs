#![allow(dead_code)]
pub use nbody::Vec3;
pub use nbody::Particle;
pub use nbody::System;

fn main() {
    let vec1 = Vec3::new(1, 1, 1);
    let vec2 = Vec3::new(1, 1, 1);
    println!("Here is a vector: {:?}.", vec1);

    let particle = Particle::new(vec1, vec2, 1);

    println!("Here is a particle: {:?}.", particle);

    let vec3 = Vec3::default();
    println!("Here is a default vector: {:?}.", vec3);

    System::record();
}
