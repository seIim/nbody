pub mod bodies {
    pub mod particle;
}
pub mod base {
    pub mod vec3;
}

pub use base::vec3::Vec3;
pub use bodies::particle::Particle;
