use anyhow::Result;
use bvh::Bvh;
use gpu::GpuContext;
use math::Vec3;
use rand::{rngs::StdRng, SeedableRng};
use scene::Scene;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SHADER: &[u8] = include_bytes!("../../../shaders/trace.spv");

struct App {
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
    bvh: Option<Bvh>,
    frame: u32,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        let attrs = Window::default_attributes().with_title("lumen").with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT));
        let win = Arc::new(el.create_window(attrs).unwrap());

        let mut sc = Scene::new();
        sc.cornell_box();
        let bvh = Bvh::build(sc.triangles.clone());

        let gpu = unsafe { GpuContext::new(win.as_ref(), SHADER, WIDTH, HEIGHT).expect("gpu init") };
        self.bvh = Some(bvh);
        self.gpu = Some(gpu);
        self.window = Some(win);
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, ev: WindowEvent) {
        match ev {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::RedrawRequested => {
                if let (Some(gpu), Some(bvh)) = (self.gpu.as_mut(), self.bvh.as_ref()) {
                    if self.frame == 0 {
                        gpu.upload_scene(bvh).ok();
                    }
                    let cam = Vec3::new(0.0, 1.0, 3.0);
                    gpu.dispatch(cam, self.frame, 4).ok();
                    if let Ok(px) = gpu.read_pixels() {
                        write_ppm(&px, WIDTH, HEIGHT, "frame.ppm").ok();
                    }
                    self.frame += 1;
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _el: &ActiveEventLoop) {
        if let Some(w) = &self.window { w.request_redraw(); }
    }
}

fn write_ppm(px: &[u8], w: u32, h: u32, path: &str) -> Result<()> {
    use std::io::Write;
    let mut f = std::fs::File::create(path)?;
    write!(f, "P6\n{} {}\n255\n", w, h)?;
    for i in 0..(w * h) as usize {
        let o = i * 16;
        let r = f32::from_le_bytes(px[o..o + 4].try_into().unwrap());
        let g = f32::from_le_bytes(px[o + 4..o + 8].try_into().unwrap());
        let b = f32::from_le_bytes(px[o + 8..o + 12].try_into().unwrap());
        f.write_all(&[
            (r / (1.0 + r) * 255.0).clamp(0.0, 255.0) as u8,
            (g / (1.0 + g) * 255.0).clamp(0.0, 255.0) as u8,
            (b / (1.0 + b) * 255.0).clamp(0.0, 255.0) as u8,
        ])?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let el = EventLoop::new()?;
    let mut app = App { window: None, gpu: None, bvh: None, frame: 0 };
    let _rng = StdRng::seed_from_u64(0);
    el.run_app(&mut app)?;
    Ok(())
}
