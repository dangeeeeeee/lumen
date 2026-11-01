use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn dot(self, o: Self) -> f32 { self.x * o.x + self.y * o.y + self.z * o.z }
    pub fn cross(self, o: Self) -> Self {
        Self::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }
    pub fn len(self) -> f32 { self.dot(self).sqrt() }
    pub fn norm(self) -> Self { let l = self.len(); Self::new(self.x / l, self.y / l, self.z / l) }
    pub fn reflect(self, n: Self) -> Self { self - n * (2.0 * self.dot(n)) }
}

impl Add for Vec3 { type Output = Self; fn add(self, o: Self) -> Self { Self::new(self.x + o.x, self.y + o.y, self.z + o.z) } }
impl Sub for Vec3 { type Output = Self; fn sub(self, o: Self) -> Self { Self::new(self.x - o.x, self.y - o.y, self.z - o.z) } }
impl Mul<f32> for Vec3 { type Output = Self; fn mul(self, s: f32) -> Self { Self::new(self.x * s, self.y * s, self.z * s) } }

#[derive(Copy, Clone, Debug)]
pub struct Mat4 { pub m: [[f32; 4]; 4] }

impl Mat4 {
    pub fn identity() -> Self {
        let mut m = [[0.0; 4]; 4];
        for i in 0..4 { m[i][i] = 1.0; }
        Self { m }
    }
    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y * 0.5).tan();
        let mut m = [[0.0; 4]; 4];
        m[0][0] = f / aspect;
        m[1][1] = f;
        m[2][2] = far / (near - far);
        m[2][3] = (far * near) / (near - far);
        m[3][2] = -1.0;
        Self { m }
    }
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = (target - eye).norm();
        let s = f.cross(up).norm();
        let u = s.cross(f);
        let mut m = Self::identity();
        m.m[0] = [s.x, u.x, -f.x, 0.0];
        m.m[1] = [s.y, u.y, -f.y, 0.0];
        m.m[2] = [s.z, u.z, -f.z, 0.0];
        m.m[3] = [-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0];
        m
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Quat { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }

impl Quat {
    pub fn identity() -> Self { Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 } }
    pub fn mul(self, o: Self) -> Self {
        Self {
            w: self.w * o.w - self.x * o.x - self.y * o.y - self.z * o.z,
            x: self.w * o.x + self.x * o.w + self.y * o.z - self.z * o.y,
            y: self.w * o.y - self.x * o.z + self.y * o.w + self.z * o.x,
            z: self.w * o.z + self.x * o.y - self.y * o.x + self.z * o.w,
        }
    }
    pub fn rotate(self, v: Vec3) -> Vec3 {
        let q = Vec3::new(self.x, self.y, self.z);
        let t = q.cross(v) * 2.0;
        v + t * self.w + q.cross(t)
    }
}
