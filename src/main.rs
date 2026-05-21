use std::sync::Arc;
use winit::{
    application::ApplicationHandler, 
    event::*, 
    event_loop::{ActiveEventLoop, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey}, 
    window::Window
};

mod physics;
mod ergodic;
use physics::*;

fn main() {
    env_logger::init();
    println!("Hello, world!");
}
