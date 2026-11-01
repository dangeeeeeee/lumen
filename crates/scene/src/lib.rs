use bvh::Triangle;
use math::{Mat4, Vec3};

#[derive(Copy, Clone, PartialEq)]
pub struct Entity(pub u32);

#[derive(Default)]
pub struct Transform { pub position: Vec3, pub rotation: Vec3, pub scale: Vec3 }

#[derive(Copy, Clone)]
pub struct Material { pub albedo: Vec3, pub emission: Vec3, pub roughness: f32 }

pub struct Camera { pub eye: Vec3, pub target: Vec3, pub up: Vec3, pub fov: f32 }

impl Camera {
    pub fn view_proj(&self, aspect: f32) -> Mat4 {
        let view = Mat4::look_at(self.eye, self.target, self.up);
        let proj = Mat4::perspective(self.fov, aspect, 0.1, 100.0);
        let mut out = Mat4::identity();
        for r in 0..4 {
            for c in 0..4 {
                let mut acc = 0.0;
                for k in 0..4 { acc += proj.m[r][k] * view.m[k][c]; }
                out.m[r][c] = acc;
            }
        }
        out
    }
}

pub struct Scene {
    pub entities: Vec<Entity>,
    pub transforms: Vec<Transform>,
    pub materials: Vec<Material>,
    pub triangles: Vec<Triangle>,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        Self { entities: Vec::new(), transforms: Vec::new(), materials: Vec::new(), triangles: Vec::new(), camera: Camera { eye: Vec3::new(0.0, 1.0, 3.0), target: Vec3::new(0.0, 1.0, 0.0), up: Vec3::new(0.0, 1.0, 0.0), fov: 1.0 } }
    }

    pub fn spawn(&mut self, t: Transform, m: Material) -> Entity {
        let id = Entity(self.entities.len() as u32);
        self.entities.push(id);
        self.transforms.push(t);
        self.materials.push(m);
        id
    }

    pub fn add_quad(&mut self, a: Vec3, b: Vec3, c: Vec3, d: Vec3, mat: u32) {
        self.triangles.push(Triangle { v0: a, v1: b, v2: c, mat_id: mat });
        self.triangles.push(Triangle { v0: a, v1: c, v2: d, mat_id: mat });
    }

    pub fn cornell_box(&mut self) {
        self.materials.push(Material { albedo: Vec3::new(0.9, 0.1, 0.1), emission: Vec3::default(), roughness: 1.0 });
        self.materials.push(Material { albedo: Vec3::new(0.1, 0.9, 0.2), emission: Vec3::default(), roughness: 1.0 });
        self.materials.push(Material { albedo: Vec3::new(0.8, 0.8, 0.8), emission: Vec3::default(), roughness: 1.0 });
        self.materials.push(Material { albedo: Vec3::new(1.0, 0.9, 0.7), emission: Vec3::new(4.0, 4.0, 4.0), roughness: 1.0 });

        self.add_quad(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(-1.0, 0.0, 1.0), Vec3::new(-1.0, 2.0, 1.0), Vec3::new(-1.0, 2.0, -1.0), 0);
        self.add_quad(Vec3::new(1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 2.0, 1.0), Vec3::new(1.0, 2.0, -1.0), 1);
        self.add_quad(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(-1.0, 0.0, 1.0), 2);
        self.add_quad(Vec3::new(-1.0, 2.0, -1.0), Vec3::new(1.0, 2.0, -1.0), Vec3::new(1.0, 2.0, 1.0), Vec3::new(-1.0, 2.0, 1.0), 2);
        self.add_quad(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0), Vec3::new(1.0, 2.0, -1.0), Vec3::new(-1.0, 2.0, -1.0), 2);
        self.add_quad(Vec3::new(-0.4, 1.99, -0.4), Vec3::new(0.4, 1.99, -0.4), Vec3::new(0.4, 1.99, 0.4), Vec3::new(-0.4, 1.99, 0.4), 3);
    }
}

impl Default for Scene { fn default() -> Self { Self::new() } }
