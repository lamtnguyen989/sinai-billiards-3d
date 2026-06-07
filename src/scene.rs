use glam::{Mat4, Vec3};

/***
*   Camera and shaders 
***/

// Scene camera
#[repr(C)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform
{
    pub view_proj:  [[f32; 4]; 4],
    pub view:       [[f32; 4]; 4],
    pub proj:       [[f32; 4]; 4],
    pub camera_pos: [f32; 4],
    pub time:       f32,
}

impl CameraUniform
{
    pub fn new() -> Self {
        return Self::default();
    }
}

// Orbiting data around the sphere center
#[derive(Clone, Copy)]
pub struct OrbitCamera
{
    pub target:         Vec3,   // The target coordinate where camera will be orbiting
    pub yaw:            f32,
    pub pitch:          f32,
    pub distance:       f32,
    pub fov_y:          f32,
    pub aspect_ratio:   f32,
}

impl OrbitCamera
{
    // Constructors
    pub fn new(box_size: f32, aspect_ratio: f32) -> Self {
        return Self {
            target:         Vec3::splat(0.5*box_size),  // Box center
            pitch:          -0.5,
            yaw:            0.5,
            distance:       2.5*box_size,
            fov_y:          std::f32::consts::FRAC_PI_4,
            aspect_ratio:   aspect_ratio
        }
    }

    // Converting the spherical coordinates to physical space (Cartesian) position
    // IMPORTANT: We working in the spherical frame where Y-axis is up (elevation from XZ-plane)
    pub fn physical_position(&self) -> Vec3 {
        let (cos_yaw, sin_yaw) = (f32::cos(self.yaw), f32::sin(self.yaw));
        let (cos_pitch, sin_pitch) = (f32::cos(self.pitch), f32::sin(self.pitch));

        let direction = Vec3::new(cos_pitch*cos_yaw, sin_pitch, cos_pitch*sin_yaw);
        return direction.mul_add(Vec3::splat(self.distance), self.target);
    }

    // Orbitting mechanism
    pub fn orbit(&mut self, delta_x: f32, delta_y: f32) {
        // Hard-coding numerical practicality factors (for now, dynamic way possible? But do I want to bother?)
        let SENSITIVITY = 0.005;    
        let radians_range = std::f32::consts::FRAC_PI_2 - SENSITIVITY;
        
        self.yaw += SENSITIVITY * delta_x;
        self.pitch = (self.pitch + delta_y*SENSITIVITY)
                        .clamp(-radians_range, radians_range);  // Coupled with the sensitivity for no reason, but fix in prod ig
    }

    // Convert orbit camera data to uniform data
    pub fn to_uniform(&mut self, time: f32) -> CameraUniform {
        // Position and perspectives
        let position = self.physical_position();
        let view = Mat4::look_at_rh(position, self.target, Vec3::Y);
        let proj = Mat4::perspective_rh(self.fov_y, self.aspect_ratio, 0.01, 100.0);

        return CameraUniform {
            view_proj:  (view * proj).to_cols_array_2d(),
            view:       view.to_cols_array_2d(),
            proj:       proj.to_cols_array_2d(),
            camera_pos: [position.x, position.y, position.z, 1.0],
            time:       time
        };
    }
}

// Render data: Trajectory line
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
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
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SphereData
{
    pub position:   [f32; 3],
    pub normal:     [f32; 3],
}

impl SphereData
{
    pub fn new() -> Self {
        todo!();
    }
}


// Render data: Box
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BoxData
{
    pub position:   [f32; 3],
}

impl BoxData
{
    pub fn new() -> Self {
        todo!();
    }
}