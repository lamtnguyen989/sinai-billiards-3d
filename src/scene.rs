use glam::{Mat4, Vec3};
use bytemuck::{Pod, Zeroable};

/***
*   Camera and shaders 
***/

// Scene camera
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform
{
    pub view_proj:  [[f32; 4]; 4],
    pub view:       [[f32; 4]; 4],
    pub proj:       [[f32; 4]; 4],
    pub time:       f32,
}

#[derive(Default)]
pub struct Camera
{
    pub yaw:    f32,
    pub pitch:  f32,
}

impl Camera
{
    pub fn new() -> Self {
        todo!();
    }
}

// Render data: Trajectory line
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LineData
{
    pub position:   [f32; 3],
    pub color:      [f32; 4],
}

impl LineData
{

}

// Render data: Sphere
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SphereData
{
    pub position:   [f32; 3],
    pub color:      [f32; 4],
}

impl SphereData
{
    pub fn new() -> Self {
        todo!();
    }
}


// Render data: Box
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BoxData
{
    pub position:   [f32; 3],
    pub color:      [f32; 4],
}

impl BoxData
{
    pub fn new() -> Self {
        todo!();
    }
}