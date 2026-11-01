use math::{Quat, Vec3};

pub struct RigidBody {
    pub pos: Vec3,
    pub vel: Vec3,
    pub inv_mass: f32,
    pub inv_inertia: f32,
    pub orient: Quat,
    pub ang_vel: Vec3,
}

pub fn gjk<F, G>(mut support_a: F, mut support_b: G) -> bool
where
    F: FnMut(Vec3) -> Vec3,
    G: FnMut(Vec3) -> Vec3,
{
    let mut dir = Vec3::new(1.0, 0.0, 0.0);
    let mut simplex = [Vec3::default(); 4];
    let mut n = 0usize;
    let a = support_a(dir) - support_b(dir * -1.0);
    simplex[0] = a;
    n = 1;
    dir = a * -1.0;

    loop {
        let a = support_a(dir) - support_b(dir * -1.0);
        if a.dot(dir) < 0.0 { return false; }
        simplex[n] = a;
        n += 1;
        if do_simplex(&mut simplex, &mut n, &mut dir) { return true; }
    }
}

fn do_simplex(s: &mut [Vec3; 4], n: &mut usize, dir: &mut Vec3) -> bool {
    match *n {
        2 => {
            let a = s[1];
            let b = s[0];
            let ab = b - a;
            let ao = a * -1.0;
            *dir = ab.cross(ao).cross(ab);
            if dir.len() < 1e-6 { *dir = ab.cross(Vec3::new(0.0, 1.0, 0.0)); }
            false
        }
        3 => {
            let a = s[2];
            let b = s[1];
            let c = s[0];
            let ab = b - a;
            let ac = c - a;
            let ao = a * -1.0;
            let abc = ab.cross(ac);
            if abc.cross(ac).dot(ao) > 0.0 {
                if ac.dot(ao) > 0.0 {
                    s[1] = c;
                    *n = 2;
                    *dir = ac.cross(ao).cross(ac);
                } else if ab.dot(ao) > 0.0 {
                    *n = 2;
                    *dir = ab.cross(ao).cross(ab);
                } else {
                    *n = 1;
                    *dir = ao;
                }
            } else if ab.cross(abc).dot(ao) > 0.0 {
                *n = 2;
                *dir = ab.cross(ao).cross(ab);
            } else if abc.dot(ao) > 0.0 {
                *dir = abc;
            } else {
                s[1] = b;
                s[2] = a;
                *dir = abc * -1.0;
            }
            false
        }
        4 => {
            let a = s[3];
            let b = s[2];
            let c = s[1];
            let d = s[0];
            let ao = a * -1.0;
            let abc = (b - a).cross(c - a);
            let acd = (c - a).cross(d - a);
            let adb = (d - a).cross(b - a);
            if abc.dot(ao) > 0.0 {
                s[0] = c;
                s[1] = b;
                *n = 3;
                *dir = abc;
                return false;
            }
            if acd.dot(ao) > 0.0 {
                s[0] = d;
                s[1] = c;
                *n = 3;
                *dir = acd;
                return false;
            }
            if adb.dot(ao) > 0.0 {
                s[0] = b;
                s[1] = d;
                *n = 3;
                *dir = adb;
                return false;
            }
            true
        }
        _ => false,
    }
}

pub struct Cloth {
    pub pos: Vec<Vec3>,
    pub prev: Vec<Vec3>,
    pub constraints: Vec<(usize, usize, f32)>,
    pub pinned: Vec<usize>,
}

impl Cloth {
    pub fn step(&mut self, gravity: Vec3, dt: f32, iterations: u32) {
        for i in 0..self.pos.len() {
            if self.pinned.contains(&i) { continue; }
            let v = (self.pos[i] - self.prev[i]) * 0.99;
            self.prev[i] = self.pos[i];
            self.pos[i] = self.pos[i] + v + gravity * (dt * dt);
        }
        for _ in 0..iterations {
            for &(i, j, rest) in &self.constraints {
                let d = self.pos[j] - self.pos[i];
                let dist = d.len().max(1e-6);
                let diff = (dist - rest) / dist * 0.5;
                if !self.pinned.contains(&i) { self.pos[i] = self.pos[i] + d * diff; }
                if !self.pinned.contains(&j) { self.pos[j] = self.pos[j] - d * diff; }
            }
        }
    }
}
