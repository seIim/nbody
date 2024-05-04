#![allow(dead_code)]
pub mod bodies {
    pub mod particle;
}
pub mod base {
    pub mod vec3;
}

pub use base::vec3::Vec3;
pub use bodies::particle::Particle;

fn main() {
    let vec1 = Vec3::new(1, 1, 1);
    let vec2 = Vec3::new(1, 1, 1);
    println!("Here is a vector: {:?}.", vec1);

    let particle = Particle::new(vec1, vec2, 1);

    println!("Here is a particle: {:?}.", particle);

    println!("Here is the partiles energy: {:?}", particle.get_t());

    let vec3 = Vec3::default();
    println!("Here is a default vector: {:?}.", vec3);
}
