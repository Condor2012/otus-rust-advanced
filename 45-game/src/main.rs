use std::{sync::Arc, time::Instant};

use rand::{RngExt as _, SeedableRng, rngs::SmallRng};
use wgpu::util::DeviceExt as _;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window,
};


const W: f32 = 480.0;
const H: f32 = 720.0;
const PLATFORM_W: f32 = 84.0;
const PLAYER_W: f32 = 32.0;
const GRAVITY: f32 = 1600.0;
const JUMP: f32 = 650.0;
const START_Y: f32 = 80.0;
const RNG_SEED: u64 = 42;
const PLATFORM_SPACING: f32 = 80.0;
const PLATFORM_SPAWN_MARGIN: f32 = 80.0;
const PLATFORM_MAX_STEP: i32 = 120;
const MOVE_SPEED: f32 = 300.0;
const CAMERA_FOLLOW_HEIGHT: f32 = 0.55;
const PLATFORM_CULL_MARGIN: f32 = 20.0;
const GAME_OVER_MARGIN: f32 = 40.0;
const MAX_FRAME_DT: f32 = 1.0 / 30.0;

const GRID_SPACING: u32 = 24;
const GRID_LINE_WIDTH: f32 = 1.0;
const OUTLINE_WIDTH: f32 = 2.0;
const PLATFORM_HEIGHT: f32 = 13.0;
const PLATFORM_FILL_HEIGHT: f32 = 7.0;
const PLAYER_BODY_HEIGHT: f32 = 30.0;
const PLAYER_FOOT_WIDTH: f32 = 7.0;
const PLAYER_FOOT_HEIGHT: f32 = 8.0;
const PLAYER_HEIGHT: f32 = PLAYER_BODY_HEIGHT + PLAYER_FOOT_HEIGHT;
const PLAYER_FOOT_OFFSETS: [f32; 2] = [5.0, 21.0];
const PLAYER_EYE_OFFSETS: [f32; 2] = [9.0, 22.0];
const PLAYER_EYE_Y: f32 = 6.0;
const PLAYER_EYE_WIDTH: f32 = 4.0;
const PLAYER_EYE_HEIGHT: f32 = 6.0;
const GAME_OVER_LINE_HEIGHT: f32 = 8.0;
const INK_COLOR: [f32; 3] = [0.08, 0.13, 0.09];
const GRID_COLOR: [f32; 3] = [0.82, 0.86, 0.80];
const PLATFORM_COLOR: [f32; 3] = [0.23, 0.65, 0.16];
const PLAYER_COLOR: [f32; 3] = [0.58, 0.76, 0.12];
const GAME_OVER_COLOR: [f32; 3] = [0.75, 0.15, 0.10];
const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.96,
    g: 0.95,
    b: 0.85,
    a: 1.0,
};

struct Platform {
    x: f32,
    y: f32,
}

struct Game {
    x: f32,
    y: f32, // Feet position; world Y increases upward.
    vy: f32,
    camera: f32,
    score: u32,
    platforms: Vec<Platform>,
    rng: rand::rngs::SmallRng,
    over: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            x: W / 2.0,
            y: START_Y,
            vy: JUMP,
            camera: 0.0,
            score: 0,
            platforms: vec![Platform {
                x: W / 2.0 - PLATFORM_W / 2.0,
                y: START_Y,
            }],
            rng: SmallRng::seed_from_u64(RNG_SEED),
            over: false,
        };
        game.fill_platforms();
        game
    }

    fn fill_platforms(&mut self) {
        while self.platforms.last().unwrap().y < self.camera + H + PLATFORM_SPAWN_MARGIN {
            let last = self.platforms.last().unwrap();
            let step = self.rng.random_range(-PLATFORM_MAX_STEP..=PLATFORM_MAX_STEP) as f32;
            self.platforms.push(Platform { x: (last.x + step).clamp(0.0, W - PLATFORM_W), y: last.y + PLATFORM_SPACING});
        }
    }

    fn update(&mut self, dt: f32, direction: f32) {
        if self.over {
            return;
        }
        self.x = (self.x + direction * MOVE_SPEED * dt).rem_euclid(W);
        let old_y = self.y;
        self.vy -= GRAVITY * dt;
        self.y += self.vy * dt;

        if self.vy < 0.0 {
            for platform in self.platforms.iter().rev() {
                let overlaps = [-W, 0.0, W].iter().any(|offset| {
                    let x = self.x + offset;
                    x + PLAYER_W / 2.0 > platform.x && x - PLAYER_W / 2.0 < platform.x + PLATFORM_W
                });
                if old_y >= platform.y && self.y <= platform.y && overlaps{
                    self.y = platform.y;
                    self.vy = JUMP;
                    break;
                }
            }
        }
        self.camera = self.camera.max(self.y - H * CAMERA_FOLLOW_HEIGHT);
        self.score = self.score.max((self.y - START_Y).max(0.0) as u32);
        self.platforms.retain(|p| p.y > self.camera - PLATFORM_CULL_MARGIN);
        self.fill_platforms();
        self.over = self.y < self.camera - GAME_OVER_MARGIN;
    }

    fn vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        let mut rect = |x, y, w, h, color| {
            for [dx, dy] in [
                [0.0, 0.0],
                [1.0, 0.0],
                [0.0, 1.0],
                [0.0, 1.0],
                [1.0, 0.0],
                [1.0, 1.0],
            ] {
                vertices.push(Vertex {
                    position: [2.0 * (x + dx * w) / W - 1.0, 1.0 - 2.0 * (y + dy * h) / H],
                    color,
                });
            }
        };

        for x in (0..W as u32).step_by(GRID_SPACING as usize) {
            rect(x as f32, 0.0, GRID_LINE_WIDTH, H, GRID_COLOR);
        }
        for y in (0..H as u32 + GRID_SPACING).step_by(GRID_SPACING as usize) {
            rect(
                0.0,
                y as f32 + self.camera % GRID_SPACING as f32 - GRID_SPACING as f32,
                W,
                GRID_LINE_WIDTH,
                GRID_COLOR,
            );
        }

        for p in &self.platforms {
            let y = H - (p.y - self.camera);
            rect(p.x, y, PLATFORM_W, PLATFORM_HEIGHT, INK_COLOR);
            rect(
                p.x + OUTLINE_WIDTH,
                y + OUTLINE_WIDTH,
                PLATFORM_W - 2.0 * OUTLINE_WIDTH,
                PLATFORM_FILL_HEIGHT,
                PLATFORM_COLOR,
            );
        }
        for offset in [-W, 0.0, W] {
            let x = self.x + offset - PLAYER_W / 2.0;
            let y = H - (self.y - self.camera) - PLAYER_HEIGHT;
            rect(x, y, PLAYER_W, PLAYER_BODY_HEIGHT, INK_COLOR);
            rect(
                x + OUTLINE_WIDTH,
                y + OUTLINE_WIDTH,
                PLAYER_W - 2.0 * OUTLINE_WIDTH,
                PLAYER_BODY_HEIGHT - 2.0 * OUTLINE_WIDTH,
                PLAYER_COLOR,
            );
            for offset in PLAYER_FOOT_OFFSETS {
                rect(
                    x + offset,
                    y + PLAYER_BODY_HEIGHT,
                    PLAYER_FOOT_WIDTH,
                    PLAYER_FOOT_HEIGHT,
                    INK_COLOR,
                );
            }
            for offset in PLAYER_EYE_OFFSETS {
                rect(
                    x + offset,
                    y + PLAYER_EYE_Y,
                    PLAYER_EYE_WIDTH,
                    PLAYER_EYE_HEIGHT,
                    INK_COLOR,
                );
            }
        }
        if self.over {
            rect(
                0.0,
                (H - GAME_OVER_LINE_HEIGHT) / 2.0,
                W,
                GAME_OVER_LINE_HEIGHT,
                GAME_OVER_COLOR,
            );
        }
        vertices
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

struct Renderer {
    window: Arc<Window>,
    instance: wgpu::Instance,
    device: wgpu::Device,
    surface: wgpu::Surface<'static>,
    pipeline: wgpu::RenderPipeline,
    queue: wgpu::Queue,
}

impl Renderer {
    async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("No compatible GPU found");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("Could not create GPU device");

        let size = window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .unwrap();
        config.present_mode = wgpu::PresentMode::AutoVsync;
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Colored rectangles"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline: wgpu::RenderPipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                 module: &shader,
                 entry_point: Some("vs_main"),
                 compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3],
                })],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(config.format.into())],
            }),
            multiview_mask: None,
            cache: None,
        });

        Renderer { window, instance, surface, pipeline, queue, device }
    }

    fn draw(&mut self, game: &Game) {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => frame,
            _ => return,
        };
        let view = frame.texture.create_view(&Default::default());
        let vertices = game.vertices();
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, buffer.slice(..));
            pass.draw(0..vertices.len() as u32, 0..1);
        }
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        self.queue.present(frame);
    }

}


struct App {
    renderer: Option<Renderer>,
    game: Game,
    left: bool,
    right: bool,
    last_frame: Instant,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Jump")
                    .with_inner_size(LogicalSize::new(W, H)),
            )
            .unwrap();
        self.renderer = Some(pollster::block_on(Renderer::new(Arc::new(window))));
        self.last_frame = Instant::now();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = (now - self.last_frame).as_secs_f32().min(MAX_FRAME_DT);
                self.last_frame = now;
                self.game.update(dt, self.right as u8 as f32 - self.left as u8 as f32);
                if let Some(render) = &mut self.renderer {
                    render.draw(&self.game);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state.is_pressed();
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::ArrowLeft | KeyCode::KeyA) => self.left = pressed,
                    PhysicalKey::Code(KeyCode::ArrowRight | KeyCode::KeyD) => self.right = pressed,
                    PhysicalKey::Code(KeyCode::Escape) if pressed => event_loop.exit(),
                    _ => {}
                }
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        if let Some(renderer) = &self.renderer {
            renderer.window.request_redraw();
        }
    }
}

fn main() -> Result<(), winit::error::EventLoopError> {
    EventLoop::new()?.run_app(&mut App {
        renderer: None,
        game: Game::new(),
        left: false,
        right: false,
        last_frame: Instant::now(),
    })
}
