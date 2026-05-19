use rand::Rng;
use glam::Vec3;

/* Geometry constants */
pub const BOX_SIZE      : f32 = 1.0;    // Standard cube [0,1]^3
pub const SPHERE_RADIUS : f32 = 0.25;   // Sinai billiards obstacle sphere radius
pub const SPHERE_CENTER : Vec3 = Vec3::new(0.5, 0.5, 0.5);   // Smack the sphere down the center of the box

/* Constants needed for Lyapunov spectra computations */
pub const NUM_TANGENTS  : usize = 6;    // Phase space dimension


/* Sphere reflections */
fn reflection_sphere(pos: Vec3, vel: Vec3) -> Vec3
{
    let n = (pos - SPHERE_CENTER).normalize();  // Surface normals
    let reflection = vel - 2.0*vel.dot(n) * n;
    return reflection;
}

/* Box reflection */
fn reflection_box(pos: Vec3, vel: Vec3) -> Vec3
{
    let BOX_EPSILON = 1e-5; // Error margin for being outside

    // Assuming perfect reflection of components of it step outside
    let mut v = vel;
    if (pos.x < BOX_EPSILON || pos.x > (BOX_SIZE - BOX_EPSILON)) {v.x = -v.x;}
    if (pos.y < BOX_EPSILON || pos.y > (BOX_SIZE - BOX_EPSILON)) {v.y = -v.y;}
    if (pos.z < BOX_EPSILON || pos.z > (BOX_SIZE - BOX_EPSILON)) {v.z = -v.z;}

    return v;
}