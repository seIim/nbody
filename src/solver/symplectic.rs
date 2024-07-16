/* symplectic.rs
Suite of symplectic integrators.
Following the methods shown in this review article: https://pubs.aip.org/aapt/ajp/article/73/10/938/1042416/Symplectic-integrators-An-introduction.
*/
pub use crate::gravity;
pub use crate::Particle;
pub use crate::Vec3;
pub use rayon::iter::IntoParallelRefMutIterator;
pub use rayon::iter::IndexedParallelIterator;
pub use rayon::prelude::*;

pub trait Solver {
    fn update_velocity(&self, all_ps: &mut Vec<Particle>, dt: f64);
    fn update_position(&self, all_ps: &mut Vec<Particle>, dt: f64);
    fn step(&self, all_ps: &mut Vec<Particle>, dt: f64);
}

pub struct SymplecticEuler {}

impl Solver for SymplecticEuler {

    fn update_velocity(&self, ps: &mut Vec<Particle>, dt:f64) -> () {
        let all_ps: Vec<Particle> = ps.clone();

        ps.par_iter_mut().enumerate().for_each(|(ind, p)|{   
            let a: Vec3 = gravity::Force::greedy(&p, &all_ps, ind)*(p.m);
            (*p).v = (*p).v + a*dt;
        } );
    }

    fn update_position(&self, ps: &mut Vec<Particle>, dt:f64) -> () {
        ps.par_iter_mut().for_each(|p: &mut Particle| {(*p).r = (*p).r + (*p).v*dt;});
    }

    fn step(&self, ps: &mut Vec<Particle>, dt:f64) {
        self.update_velocity(ps, dt);
        self.update_position(ps, dt);
    }
}

impl SymplecticEuler {

    // pub fn warmup (ps: &mut Vec<Particle>) -> () {
    //     let all_ps = ps.clone();

    //     for (ind, p) in ps.iter_mut().enumerate() {
    //         (*p).a[0] = gravity::Force::greedy(&p, &all_ps, ind)*(p.m);
    //     }
    // }

}

pub struct Verlet {}

