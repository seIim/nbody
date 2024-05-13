pub mod bodies {
    pub mod particle;
    pub mod system;
}
pub mod base {
    pub mod vec3;
    pub mod gravity;
}
pub mod solver {
    pub mod symplectic;
}

pub use bodies::system::System;
pub use bodies::particle::Particle;

pub use base::vec3::Vec3;
pub use base::gravity;

pub use solver::symplectic;
