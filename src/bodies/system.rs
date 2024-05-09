#![allow(dead_code)]
#![allow(unused_doc_comments)]
/**
 * PLAN: A system is a data structure which holds references to bodies.
 * Somehow, the system will be endowed with an EOM (equation of motion)
 * that tells the bodies how to move. The EOM will be integrated forward
 * and solved for the position and velocity of each body. In an n-body 
 * simulation the EOM will look like
 *                      aₙ = ∑ₖ Gmₖmₙ rₘₙ⁻².
 * Then the particle's position may update in time via a solver, e.g. 
 * with verlet integration it will be,
 *                      xₙ₊₁ = xₙ + vₙdt + 0.5aₙdt².
 * Where velocity is given by the verlet velocity formula,
 *                      vₙ₊₁ = vₙ + aₙdt.
 * In general, this equation looks like
 *                      x(t) = F[x, x'].
*/

pub use crate::Particle;
pub use crate::Vec3;

pub struct System {
    pub particles: Vec<Particle>,
}

impl System {
    pub fn new (ps: Vec<Particle>) -> Self {
        Self {
            particles: ps,
        }
    }
    // update the position and velocity of all particles according to 
    // the EOM and solver specified for the system. TODO: everything
    pub fn step(&mut self){
        // I want the solver to have the functionality: solver(&self, &p: &Particle, EOM) --> Particle
        // NOTE: The solver numerically finds xₙ₊₁ given xₙ and some symplectic integration scheme; 
        //we will have a different module for calculating the magnitude of the gravity term using greedy 
        // / barnes huts / particle mesh or whatever other algorithm seems interesting to implement.
        for p in &mut self.particles {
            p.update_r(Vec3::new(1, 1, 1));
            p.update_v(Vec3::new(1, 1, 1));
        }
    }

    pub fn record () {
        /**  TODO: Implement function that records the positions and velocities
        * and dumps to a csv file. There should be options on the time step
        * intervals it takes to record values etc.
        */ 
        println!(
        "TODO: Implement function that records the positions and velocities
        and dumps to a csv file. There should be options on the time step
        intervals it takes to record values etc.
        ");
    }
}