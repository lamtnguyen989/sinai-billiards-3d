mod tangent;
mod physics;
mod ergodic;

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

/* Constants */
const MAX_HISTORY: usize = 5;

struct State
{
    trajectory:     Trajectory,
    trail_length:   usize,
    time_elapsed:   f32,
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
