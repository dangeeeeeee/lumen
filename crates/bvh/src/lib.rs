use math::Vec3;

#[derive(Copy, Clone)]
pub struct Aabb { pub min: Vec3, pub max: Vec3 }

impl Aabb {
    pub fn empty() -> Self {
        Self {
            min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }
    pub fn expand(&mut self, p: Vec3) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.min.z = self.min.z.min(p.z);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
        self.max.z = self.max.z.max(p.z);
    }
    pub fn union(&self, o: &Self) -> Self {
        let mut r = *self;
        r.expand(o.min);
        r.expand(o.max);
        r
    }
    pub fn surface_area(&self) -> f32 {
        let d = self.max - self.min;
        if d.x < 0.0 { return 0.0; }
        2.0 * (d.x * d.y + d.y * d.z + d.z * d.x)
    }
}

#[derive(Copy, Clone)]
pub struct Triangle { pub v0: Vec3, pub v1: Vec3, pub v2: Vec3, pub mat_id: u32 }

impl Triangle {
    pub fn centroid(&self) -> Vec3 { (self.v0 + self.v1 + self.v2) * (1.0 / 3.0) }
    pub fn bounds(&self) -> Aabb {
        let mut b = Aabb::empty();
        b.expand(self.v0);
        b.expand(self.v1);
        b.expand(self.v2);
        b
    }
}

pub struct Node { pub bounds: Aabb, pub left: u32, pub right: u32, pub start: u32, pub count: u32 }

impl Node {
    pub fn is_leaf(&self) -> bool { self.count > 0 }
}

pub struct Bvh { pub nodes: Vec<Node>, pub prims: Vec<Triangle> }

impl Bvh {
    pub fn build(mut prims: Vec<Triangle>) -> Self {
        let mut nodes = Vec::new();
        let n = prims.len();
        if n > 0 { Self::build_rec(&mut prims, 0, n, &mut nodes); }
        Self { nodes, prims }
    }

    fn build_rec(prims: &mut [Triangle], start: usize, end: usize, nodes: &mut Vec<Node>) -> u32 {
        let idx = nodes.len() as u32;
        nodes.push(Node { bounds: Aabb::empty(), left: 0, right: 0, start: start as u32, count: 0 });

        let mut bounds = Aabb::empty();
        for p in &prims[start..end] { bounds = bounds.union(&p.bounds()); }
        nodes[idx as usize].bounds = bounds;

        if end - start <= 4 {
            nodes[idx as usize].count = (end - start) as u32;
            return idx;
        }

        let mut cent_bounds = Aabb::empty();
        for p in &prims[start..end] { cent_bounds.expand(p.centroid()); }

        let mut best_axis = 0usize;
        let mut best_cost = f32::INFINITY;
        let mut best_split = 0usize;

        for axis in 0..3 {
            let lo = match axis { 0 => cent_bounds.min.x, 1 => cent_bounds.min.y, _ => cent_bounds.min.z };
            let hi = match axis { 0 => cent_bounds.max.x, 1 => cent_bounds.max.y, _ => cent_bounds.max.z };
            if hi - lo < 1e-6 { continue; }

            const BINS: usize = 12;
            let mut bin_bounds = [Aabb::empty(); BINS];
            let mut bin_count = [0usize; BINS];

            for p in &prims[start..end] {
                let c = p.centroid();
                let v = match axis { 0 => c.x, 1 => c.y, _ => c.z };
                let b = (((v - lo) / (hi - lo) * BINS as f32) as usize).min(BINS - 1);
                bin_bounds[b] = bin_bounds[b].union(&p.bounds());
                bin_count[b] += 1;
            }

            let mut left_area = [0.0f32; BINS];
            let mut left_cnt = [0usize; BINS];
            let mut acc = Aabb::empty();
            let mut cnt = 0usize;
            for i in 0..BINS {
                acc = acc.union(&bin_bounds[i]);
                cnt += bin_count[i];
                left_area[i] = acc.surface_area();
                left_cnt[i] = cnt;
            }

            let mut acc = Aabb::empty();
            let mut cnt = 0usize;
            for i in (1..BINS).rev() {
                acc = acc.union(&bin_bounds[i]);
                cnt += bin_count[i];
                let cost = left_area[i - 1] * left_cnt[i - 1] as f32 + acc.surface_area() * cnt as f32;
                if cost < best_cost {
                    best_cost = cost;
                    best_axis = axis;
                    best_split = i;
                }
            }
        }

        let lo = match best_axis { 0 => cent_bounds.min.x, 1 => cent_bounds.min.y, _ => cent_bounds.min.z };
        let hi = match best_axis { 0 => cent_bounds.max.x, 1 => cent_bounds.max.y, _ => cent_bounds.max.z };
        let mid = (lo + hi) * 0.5;
        let split = prims[start..end].iter().partition_in_place(|p| {
            let c = p.centroid();
            match best_axis { 0 => c.x < mid, 1 => c.y < mid, _ => c.z < mid }
        });

        if best_split == 0 || split == 0 || split == end - start {
            let m = (start + end) / 2;
            prims[start..end].select_nth_unstable_by(m - start, |a, b| a.centroid().x.partial_cmp(&b.centroid().x).unwrap());
            let l = Self::build_rec(prims, start, m, nodes);
            let r = Self::build_rec(prims, m, end, nodes);
            nodes[idx as usize].left = l;
            nodes[idx as usize].right = r;
            return idx;
        }

        let l = Self::build_rec(prims, start, start + split, nodes);
        let r = Self::build_rec(prims, start + split, end, nodes);
        nodes[idx as usize].left = l;
        nodes[idx as usize].right = r;
        idx
    }
}
