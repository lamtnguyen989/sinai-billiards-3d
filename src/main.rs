mod tangent;
mod physics;
mod ergodic;
mod lyapunov;
mod scene;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler, 
    event::*, 
    event_loop::{ActiveEventLoop, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey}, 
    window::{Window}
};
use rand::{
    Rng, 
    RngExt, 
    SeedableRng, 
    rngs::StdRng
};

use glam::{Vec3, DVec3};

use tangent::*;
use physics::*;
use ergodic::*;
use lyapunov::*;
use scene::*;

/* Constants */
const MAX_HISTORY: usize = 5;
const STEPS_PER_FRAME: usize = 1;   // Number of update steps per rendering frame

/***
*   System state
***/
struct AppState
{
    traj:           Trajectory,
    stats:          ErgodicStats,
    frame_counter:  u64,
    trail_length:   usize,
    paused:         bool
}

impl AppState
{
    // Constructors
    fn new_random(seed: u64) -> Self {
        // Setting up state for random traj from seed
        let mut rng = StdRng::seed_from_u64(seed);
        let color = trajectory_palette()[0];

        return Self {
            traj:           random_trajectory(&mut rng, color),
            stats:          ErgodicStats::new(&[0.0; NUM_TANGENTS]),
            frame_counter:  0,
            trail_length:   MAX_HISTORY,
            paused:         true
        };
    }

    /*
    pub fn new() -> Self {

    }
    */

    // Update mechanisms
    fn update(&mut self) -> () {
        // Do nothing on paused
        if self.paused {return;}

        // Compute results between rendering frame
        for k in 0..STEPS_PER_FRAME {
            match self.traj.update(self.trail_length) {
                Ok(_)   => {},
                Err(e)  => eprintln!("Trajectory update failed. Error: {:?}", e),
            };
        };

        // Compute resulting stats
        self.stats = ErgodicStats::compute_from_trajectory(&self.traj);
        self.frame_counter += 1;
    }

    // Reset mechanisms
    fn reset(&mut self) -> () {
        todo!();
    }

    fn reset_from(&mut self, pos: Vec3, vel: Vec3) -> () {
        // Pick a new color from palllete
        let palette = trajectory_palette();
        let color = palette[(self.frame_counter as usize) % palette.len()];

        // Reset internal states
        self.traj = Trajectory::new(pos, vel, color);
        self.stats = ErgodicStats::new(&[0.0; NUM_TANGENTS]);
        self.frame_counter = 0;
    }
    
}

fn trajectory_palette() -> Vec<[f32; 4]> {
    // Set up a pre-determined color pallette for the trajectory
    return vec![
        [1.0, 0.35, 0.2,  0.9],
        [0.2, 0.8,  1.0,  0.9],
        [0.4, 1.0,  0.4,  0.9],
        [1.0, 0.85, 0.1,  0.9],
        [0.9, 0.3,  1.0,  0.9],
        [0.1, 1.0,  0.85, 0.9],
        [1.0, 0.6,  0.05, 0.9],
        [0.5, 0.5,  1.0,  0.9],
        [1.0, 0.4,  0.6,  0.9],
        [0.3, 0.9,  0.6,  0.9],
        [0.9, 0.9,  0.3,  0.9],
        [0.6, 0.3,  0.9,  0.9],
    ];
}

/***
*   Renderer data
***/
struct Renderer
{
    // GPU context
    surface:            wgpu::Surface<'static>,
    device:             wgpu::Device,
    queue:              wgpu::Queue,
    config:             wgpu::SurfaceConfiguration,
    size:               winit::dpi::PhysicalSize<u32>,
    depth_view:         wgpu::TextureView,

    // Render pipelines
    line_pipeline:      wgpu::RenderPipeline,
    sphere_pipeline:    wgpu::RenderPipeline,
    box_pipeline:       wgpu::RenderPipeline,
}

impl Renderer
{
    async fn new(window: std::sync::Arc<Window>) -> Self {
        // Creating wgpu instance 
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                backends:                   wgpu::Backends::PRIMARY,
                flags:                      wgpu::InstanceFlags::default(), // Apparently, this calls VALIDATION_INDIRECT_CALL flag and not empty()
                memory_budget_thresholds:   wgpu::MemoryBudgetThresholds::default(),
                backend_options:            wgpu::BackendOptions::default(),
                display:                    None,
            }
        );
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::HighPerformance,
                compatible_surface:     Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();

        // Creating a wgpu logical device and queue
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label:                  None,
                required_features:      wgpu::Features::default(),
                required_limits:        wgpu::Limits::default(),
                experimental_features:  wgpu::ExperimentalFeatures::default(),
                memory_hints:           wgpu::MemoryHints::Performance,
                trace:                  wgpu::Trace::Off,
            }
        ).await.unwrap();

        // Surface configuration
        let size = window.inner_size();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_texture_format = surface_capabilities.formats.iter()
                                        .find(|f| {**f == wgpu::TextureFormat::Rgba16Float}).copied()   // HDR rendering capabilities first
                                        .or_else(|| surface_capabilities.formats.iter()
                                                        .find(|f| {f.is_srgb()}).copied())              // Standard RGB if no HDR
                                        .unwrap_or(surface_capabilities.formats[0]);                    // Fall back to hardware capabilities

        let config = wgpu::SurfaceConfiguration {
            usage:                          wgpu::TextureUsages::RENDER_ATTACHMENT,
            format:                         surface_texture_format,
            width:                          size.width,
            height:                         size.height,
            present_mode:                   wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency:  2,
            alpha_mode:                     surface_capabilities.alpha_modes[0],
            view_formats:                   vec![],
        };
        surface.configure(&device, &config);

        // Depth texture view
        let depth_texture_view = make_depth_texture_view(&device, size.width, size.height);

        // Load shader file as a module
        let shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label:  Some("WGSL shaders"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shaders.wgsl").into()),
            }
        );

        // Create bindings for CameraUniform

        // Layout of the rendering pipelines

        // Writing all of the rendering pipelines


        todo!("Setup the pipelines and render-related things.");
    }
}


fn make_depth_texture_view(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
    let size_extent = wgpu::Extent3d {
        width:                  width,
        height:                 height,
        depth_or_array_layers:  1
    };

    let depth_texture = device.create_texture(
        &wgpu::TextureDescriptor {
            label:              Some("Depth View texture"),
            size:               size_extent,
            mip_level_count:    1,
            sample_count:       1,
            dimension:          wgpu::TextureDimension::D2,
            format:             wgpu::TextureFormat::Depth32Float,
            usage:              wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats:       &[],
        }
    );

    let depth_texture_view = depth_texture.create_view(
        &wgpu::TextureViewDescriptor::default() // TODO: Read docs and flesh this out
    );

    return depth_texture_view;
}

fn main() {
    // Environment logger
    env_logger::init();

    // Setup event loop and window
    let (width, height): (u32, u32) = (1920, 1080);
    let event_loop = EventLoop::new().unwrap();
    let window = std::sync::Arc::new(
        event_loop.create_window(
            Window::default_attributes()
                .with_title("3D Sinai Billiards Ergodic Dynamics")
                .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        ).unwrap()
    );

    // Setup render pipelines
    let mut renderer = pollster::block_on(Renderer::new(window.clone()));

    // Setup program states (random)
    let seed: u64 = 69;
    let mut program_state = AppState::new_random(seed);

    // Event loop
    // event_loop.run();
}

