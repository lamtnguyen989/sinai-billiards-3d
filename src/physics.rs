use rand::Rng;
use glam::Vec3;

/* Geometry constants */
pub const BOX_SIZE      : f32 = 1.0;    // Standard cube [0,1]^3
pub const SPHERE_RADIUS : f32 = 0.25;   // Sinai billiards obstacle sphere radius
pub const SPHERE_CENTER : Vec3 = Vec3::new(0.5, 0.5, 0.5);   // Smack the sphere down the center of the box

/* Constants needed for Lyapunov spectra computations */
pub const NUM_TANGENTS  : usize = 6;    // Phase space dimension
