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

use physics::*;
use ergodic::*;

fn main() {
    env_logger::init();
    println!("Hello, world!");
}
