/* symplectic.rs
Suite of symplectic integrators.
Following the methods shown in this review article: https://pubs.aip.org/aapt/ajp/article/73/10/938/1042416/Symplectic-integrators-An-introduction.
*/
pub use crate::gravity;
pub use crate::Particle;
pub use crate::Vec3;

//     // also kick-drift-kick leapfrog integration
//     pub fn verlet_second_order (all_ps: &Vec<Particle>, dt: f64) -> Vec<Particle>{

//         for (index, p) in all_ps.iter_mut().enumerate() {
//             *p = f(p, &all_particles, DT, index);
//         }

//         // TODO: make a generic search function that gets passed to the solver
//         let (fx, fy, fz) = gravity::Force::greedy(p, all_ps, ind);
//         let ax: f64 = fx*p.m;
//         let ay: f64 = fy*p.m;
//         let az: f64 = fz*p.m;
//         // calculate new position
//         let x: f64 = p.r.x + p.v.x*dt + 0.5*ax*dt*dt;
//         let y: f64 = p.r.y + p.v.y*dt + 0.5*ay*dt*dt;
//         let z: f64 = p.r.z + p.v.z*dt + 0.5*az*dt*dt;
//         let _r: Vec3 = Vec3::new(x,y,z);
//         let mut new_p: Particle = p.clone();
//         new_p.update_r(_r);
//         // recalculate acceleration
//         let (fx, fy, fz) = gravity::Force::greedy(&new_p, all_ps, ind);
//         let new_ax: f64 = fx*p.m;
//         let new_ay: f64 = fy*p.m;
//         let new_az: f64 = fz*p.m;
//         // then new velocity
//         let vx = p.v.x +(ax + new_ax)*(0.5*dt);
//         let vy = p.v.y +(ay + new_ay)*(0.5*dt);
//         let vz = p.v.z +(az + new_az)*(0.5*dt);
//         let _v: Vec3 = Vec3::new(vx,vy,vz);
//         // return new particle
//         return vec![Particle::new(_r, _v, [Vec3::default(), Vec3::default()], p.m)];
//     }
// }
pub trait Solver {
    fn update_velocity(&self, all_ps: &mut Vec<Particle>, dt: f64);
    fn update_position(&self, all_ps: &mut Vec<Particle>, dt: f64);
    fn step(&self, all_ps: &mut Vec<Particle>, dt: f64);
}

pub struct SymplecticEuler {}

impl Solver for SymplecticEuler {

    fn update_velocity(&self, ps: &mut Vec<Particle>, dt:f64) -> () {
        let all_ps = ps.clone();

        for (ind, p) in ps.iter_mut().enumerate() {
            let a = gravity::Force::greedy(&p, &all_ps, ind)*(p.m);
            (*p).v = (*p).v + a*dt;
        }
    }

    fn update_position(&self, ps: &mut Vec<Particle>, dt:f64) -> () {

        for p in ps.iter_mut(){
            (*p).r = (*p).r + (*p).v*dt;
        }
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

