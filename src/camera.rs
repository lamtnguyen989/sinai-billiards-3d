use glam::{Mat4, Vec3};
use bytemuck::{Pod, Zeroable};

// Scene camera
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraDataGPU
{
    pub view_proj:  [[f32; 4]; 4],
    pub view:       [[f32; 4]; 4],
    pub proj:       [[f32; 4]; 4],
    pub time:       f32,
}

#[derive(Default)]
pub struct SceneData
{
    // TODO
}

// Render data: Trajectory line
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LineVertex
{
    // TODO
}

// Render data: Sphere
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SphereVertex
{
    // TODO
}


// Render data: Box
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BoxVertex
{
    // TODO
}
