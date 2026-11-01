use bvh::{Bvh, Triangle};
use math::Vec3;
use rand::Rng;

pub struct Hit { pub t: f32, pub p: Vec3, pub n: Vec3, pub mat: u32 }

fn ray_aabb(min: Vec3, max: Vec3, o: Vec3, inv_d: Vec3) -> bool {
    let t0 = (min - o) * inv_d;
    let t1 = (max - o) * inv_d;
    let tmin = t0.x.min(t1.x).max(t0.y.min(t1.y)).max(t0.z.min(t1.z));
    let tmax = t0.x.max(t1.x).min(t0.y.max(t1.y)).min(t0.z.max(t1.z));
    tmax >= tmin && tmax >= 0.0
}

fn intersect_tri(t: &Triangle, o: Vec3, d: Vec3) -> Option<(f32, Vec3)> {
    let e1 = t.v1 - t.v0;
    let e2 = t.v2 - t.v0;
    let pv = d.cross(e2);
    let det = e1.dot(pv);
    if det.abs() < 1e-7 { return None; }
    let inv = 1.0 / det;
    let tv = o - t.v0;
    let u = tv.dot(pv) * inv;
    if !(0.0..=1.0).contains(&u) { return None; }
    let qv = tv.cross(e1);
    let v = d.dot(qv) * inv;
    if v < 0.0 || u + v > 1.0 { return None; }
    let tt = e2.dot(qv) * inv;
    if tt <= 1e-4 { return None; }
    Some((tt, e1.cross(e2).norm()))
}

pub fn intersect(bvh: &Bvh, o: Vec3, d: Vec3) -> Option<Hit> {
    if bvh.nodes.is_empty() { return None; }
    let inv_d = Vec3::new(1.0 / d.x, 1.0 / d.y, 1.0 / d.z);
    let mut stack = [0u32; 64];
    let mut sp = 0usize;
    stack[sp] = 0;
    sp += 1;
    let mut best = f32::INFINITY;
    let mut hit = None;

    while sp > 0 {
        sp -= 1;
        let n = &bvh.nodes[stack[sp] as usize];
        if !ray_aabb(n.bounds.min, n.bounds.max, o, inv_d) { continue; }
        if n.is_leaf() {
            for i in n.start as usize..(n.start + n.count) as usize {
                if let Some((t, nn)) = intersect_tri(&bvh.prims[i], o, d) {
                    if t < best {
                        best = t;
                        hit = Some(Hit { t, p: o + d * t, n: nn, mat: bvh.prims[i].mat_id });
                    }
                }
            }
        } else {
            stack[sp] = n.left; sp += 1;
            stack[sp] = n.right; sp += 1;
        }
    }
    hit
}

fn cosine_hemisphere(n: Vec3, rng: &mut impl Rng) -> Vec3 {
    let a = if n.x.abs() > 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
    let t = n.cross(a).norm();
    let b = n.cross(t);
    let u1: f32 = rng.gen();
    let u2: f32 = rng.gen();
    let r = u1.sqrt();
    let phi = 2.0 * std::f32::consts::PI * u2;
    (t * (r * phi.cos()) + b * (r * phi.sin()) + n * (1.0 - u1).sqrt()).norm()
}

fn sky(d: Vec3) -> Vec3 {
    let t = 0.5 * (d.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

pub fn trace(bvh: &Bvh, o: Vec3, d: Vec3, max_depth: u32, rng: &mut impl Rng) -> Vec3 {
    let mut radiance = Vec3::new(0.0, 0.0, 0.0);
    let mut throughput = Vec3::new(1.0, 1.0, 1.0);
    let mut o = o;
    let mut d = d;

    for depth in 0..max_depth {
        match intersect(bvh, o, d) {
            None => {
                let s = sky(d);
                radiance = radiance + Vec3::new(throughput.x * s.x, throughput.y * s.y, throughput.z * s.z);
                break;
            }
            Some(h) => {
                let albedo = match h.mat {
                    0 => Vec3::new(0.9, 0.1, 0.1),
                    1 => Vec3::new(0.1, 0.9, 0.2),
                    _ => Vec3::new(0.8, 0.8, 0.8),
                };
                if depth >= 3 {
                    let p = throughput.x.max(throughput.y).max(throughput.z).min(0.95);
                    if rng.gen::<f32>() > p { break; }
                    throughput = throughput * (1.0 / p);
                }
                let n = if d.dot(h.n) < 0.0 { h.n } else { h.n * -1.0 };
                d = cosine_hemisphere(n, rng);
                o = h.p + n * 1e-4;
                throughput = Vec3::new(throughput.x * albedo.x, throughput.y * albedo.y, throughput.z * albedo.z);
            }
        }
    }
    radiance
}

pub fn render_frame(bvh: &Bvh, cam_origin: Vec3, px: &mut [Vec3], w: u32, h: u32, spp: u32, rng: &mut impl Rng) {
    for y in 0..h {
        for x in 0..w {
            let mut acc = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..spp {
                let u = (x as f32 + rng.gen::<f32>()) / w as f32;
                let v = (y as f32 + rng.gen::<f32>()) / h as f32;
                let d = Vec3::new(u * 2.0 - 1.0, v * 2.0 - 1.0, 1.0).norm();
                acc = acc + trace(bvh, cam_origin, d, 8, rng);
            }
            px[(y * w + x) as usize] = acc * (1.0 / spp as f32);
        }
    }
}
