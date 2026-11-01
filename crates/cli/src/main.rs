use bvh::{Bvh, Triangle};
use math::Vec3;
use rand::{rngs::StdRng, SeedableRng};
use std::fs::File;
use std::io::Write;

fn cornell_box() -> Vec<Triangle> {
    let mut t = Vec::new();
    let quads = [
        (Vec3::new(-1.0, 0.0, -1.0), Vec3::new(-1.0, 0.0, 1.0), Vec3::new(-1.0, 2.0, 1.0), Vec3::new(-1.0, 2.0, -1.0), 0),
        (Vec3::new(1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 2.0, 1.0), Vec3::new(1.0, 2.0, -1.0), 1),
        (Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(-1.0, 0.0, 1.0), 2),
        (Vec3::new(-1.0, 2.0, -1.0), Vec3::new(1.0, 2.0, -1.0), Vec3::new(1.0, 2.0, 1.0), Vec3::new(-1.0, 2.0, 1.0), 2),
        (Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0), Vec3::new(1.0, 2.0, -1.0), Vec3::new(-1.0, 2.0, -1.0), 2),
    ];
    for (a, b, c, d, m) in quads {
        t.push(Triangle { v0: a, v1: b, v2: c, mat_id: m });
        t.push(Triangle { v0: a, v1: c, v2: d, mat_id: m });
    }
    t
}

fn tonemap(x: f32) -> u8 { (x / (1.0 + x) * 255.0).clamp(0.0, 255.0) as u8 }

fn main() {
    let w = 640u32;
    let h = 480u32;
    let spp = 64u32;

    let bvh = Bvh::build(cornell_box());
    let mut px = vec![Vec3::default(); (w * h) as usize];
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);

    tracer::render_frame(&bvh, Vec3::new(0.0, 1.0, 3.0), &mut px, w, h, spp, &mut rng);

    let mut f = File::create("out.ppm").unwrap();
    write!(f, "P3\n{} {}\n255\n", w, h).unwrap();
    for p in &px {
        write!(f, "{} {} {}\n", tonemap(p.x), tonemap(p.y), tonemap(p.z)).unwrap();
    }
}
