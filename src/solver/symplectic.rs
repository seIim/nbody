use std::default;

/* symplectic.rs
Suite of symplectic integrators.
Following the methods shown in this review article: https://pubs.aip.org/aapt/ajp/article/73/10/938/1042416/Symplectic-integrators-An-introduction.
*/
pub use crate::gravity;
pub use crate::Particle;
pub use crate::Vec3;

pub struct Solver {
}

impl Solver {
    
    pub fn verlet_first_order (p: &Particle, all_ps: &Vec<Particle>, dt: f64, ind: usize) -> Particle{
        //calculate acceleration (and therefore force) on the body
        let particles_clone = all_ps.clone(); // Clone the particles vector

        let mut others: Vec<_> = Vec::new();
        others.extend_from_slice(&particles_clone[..ind]);
        others.extend_from_slice(&particles_clone[ind + 1..]);
        let a = gravity::greedy(p, &others);

        // calculate new velocity first
        let vx = p.v.x + a*dt;
        let vy = p.v.y + a*dt;
        let vz = p.v.z + a*dt;
        let _v = Vec3::new(vx,vy,vz);

        // then calculate new position
        let x = p.r.x + vx*dt;
        let y = p.r.y + vy*dt;
        let z = p.r.z + vz*dt;
        let _r = Vec3::new(x,y,z);
        
        // return new particle
        return Particle::new(_r, _v, p.m);
    }
}