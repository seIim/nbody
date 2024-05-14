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
        // TODO: make a generic search function that gets passed to the solver
        let (fx, fy, fz) = gravity::Force::greedy(p, all_ps, ind);
        let ax: f64 = fx*p.m;
        let ay: f64 = fy*p.m;
        let az: f64 = fz*p.m;
        // calculate new velocity first
        let vx: f64 = p.v.x + ax*dt;
        let vy: f64 = p.v.y + ay*dt;
        let vz: f64 = p.v.z + az*dt;
        let _v: Vec3 = Vec3::new(vx,vy,vz);
        // then calculate new position
        let x: f64 = p.r.x + vx*dt;
        let y: f64 = p.r.y + vy*dt;
        let z: f64 = p.r.z + vz*dt;
        let _r: Vec3 = Vec3::new(x,y,z);

        // return new particle
        return Particle::new(_r, _v, [Vec3::default(), Vec3::default()], p.m);
    }

    // also kick-drift-kick leapfrog integration
    pub fn verlet_second_order (p: &Particle, all_ps: &Vec<Particle>, dt: f64, ind: usize) -> Particle{
        // TODO: make a generic search function that gets passed to the solver
        let (fx, fy, fz) = gravity::Force::greedy(p, all_ps, ind);
        let ax: f64 = fx*p.m;
        let ay: f64 = fy*p.m;
        let az: f64 = fz*p.m;
        // calculate new position
        let x: f64 = p.r.x + p.v.x*dt + 0.5*ax*dt*dt;
        let y: f64 = p.r.y + p.v.y*dt + 0.5*ay*dt*dt;
        let z: f64 = p.r.z + p.v.z*dt + 0.5*az*dt*dt;
        let _r: Vec3 = Vec3::new(x,y,z);
        let mut new_p: Particle = p.clone();
        new_p.update_r(_r);
        // recalculate acceleration
        let (fx, fy, fz) = gravity::Force::greedy(&new_p, all_ps, ind);
        let new_ax: f64 = fx*p.m;
        let new_ay: f64 = fy*p.m;
        let new_az: f64 = fz*p.m;
        // then new velocity
        let vx = p.v.x +(ax + new_ax)*(0.5*dt);
        let vy = p.v.y +(ay + new_ay)*(0.5*dt);
        let vz = p.v.z +(az + new_az)*(0.5*dt);
        let _v: Vec3 = Vec3::new(vx,vy,vz);
        // return new particle
        return Particle::new(_r, _v, [Vec3::default(), Vec3::default()], p.m);
    }
}