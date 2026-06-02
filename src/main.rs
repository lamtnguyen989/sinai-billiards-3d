mod tangent;
mod physics;
mod ergodic;
mod lyapunov;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler, 
    event::*, 
    event_loop::{ActiveEventLoop, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey}, 
    window::Window
};

use tangent::*;
use physics::*;
use ergodic::*;
use lyapunov::*;

/* Constants */
const MAX_HISTORY: usize = 5;

struct State
{
    trajectory:     Trajectory,
    trail_length:   usize
}

impl State
{
    /*
    pub fn new() -> Self {

    }
    */
}

fn main() {
    env_logger::init();
    println!("Hello, world!");
}
