pub mod bodies {
    pub mod particle;
    pub mod system;
}
pub mod base {
    pub mod vec3;
    pub mod gravity;
}

pub use bodies::system::System;
pub use base::vec3::Vec3;
pub use base::gravity::greedy;
pub use bodies::particle::Particle;
