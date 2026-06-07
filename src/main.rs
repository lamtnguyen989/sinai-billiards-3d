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
struct State
{
    traj:           Trajectory,
    stats:          ErgodicStats,
    frame_counter:  u64,
    trail_length:   usize,
    paused:         bool
}

impl State
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
    // Render pipelines
    line_pipeline:      wgpu::RenderPipeline,
    sphere_pipeline:    wgpu::RenderPipeline,
    box_pipeline:       wgpu::RenderPipeline,
}

impl Renderer
{
    async fn new(window: std::sync::Arc<Window>) -> Self {
        todo!("Setup the pipelines and render-related things.");
    }
}

fn main() {
    // Environment logger
    env_logger::init();

    // Setup event loop and window
    let (width, height): (u16, u16) = (1920, 1080);
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
    let mut program_state = State::new_random(seed);

    // Event loop
    // event_loop.run();
}

