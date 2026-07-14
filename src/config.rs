use glam::{Vec3};

/// Shader Type enum
#[derive(Debug, Default, Clone, Copy, clap::ValueEnum)]
pub enum ShaderType
{
    #[default] Wgsl,
    Spirv
}

/// CLI Arguments
#[derive(clap::Parser, Default, Debug, Clone, Copy)]
pub struct Args
{
    /// Box size
    #[arg(long, short, default_value_t = 1.0)]
    box_size: f32,

    /// Spherical scatterer radius
    #[arg(long, short, default_value_t = 0.25)]
    radius: f32,

    /// Particle trajectory collision history on display
    #[arg(long, default_value_t = 10)]
    pub history: usize,

    /// Simulation steps per rendering frame
    #[arg(long, default_value_t = 1)]
    pub steps_per_frame: usize,

    /// Static shader source type
    #[arg(long, value_enum, default_value_t = ShaderType::Wgsl)]
    pub shader_type: ShaderType,
}

/// Physics config
#[derive(Copy, Clone)]
pub struct PhysicsConfig
{
    box_size:       f32,
    sphere_radius:  f32,
    sphere_center:  Vec3,
}

impl From<Args> for PhysicsConfig
{
    fn from(args: Args) -> Self {
        return Self {
            box_size:       args.box_size,
            sphere_radius:  args.radius,
            sphere_center:  Vec3::splat(args.box_size / 2.0),
        }
    }
}

impl PhysicsConfig
{
    /// Constructor mainly for testing
    #[allow(dead_code)]
    pub fn new(box_size: f32, radius: f32) -> Self {
        return Self {
            box_size:       box_size,
            sphere_radius:  radius,
            sphere_center:  Vec3::splat(box_size / 2.0),
        }
    }

    pub fn box_size(&self) -> f32 {return self.box_size;}
    pub fn sphere_radius(&self) -> f32 {return self.sphere_radius;}
    pub fn sphere_center(&self) -> Vec3 {return self.sphere_center;}
}
