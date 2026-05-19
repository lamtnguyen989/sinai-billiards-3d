use rand::Rng;
use glam::Vec3;

/*** 
*   Geometry macros 
***/
pub const BOX_SIZE      : f32 = 1.0;
pub const SPHERE_RADIUS : f32 = 0.25;
pub const SPHERE_CENTER : Vec3 = Vec3::new(0.5, 0.5, 0.5);
const PHYS_EPSILON      : f32 = 1e-5;   // Physics error margin

/***
*   Reflections 
***/

// Sphere reflections
fn reflection_sphere(pos: Vec3, vel: Vec3) -> Vec3
{
    let n = (pos - SPHERE_CENTER).normalize();  // Surface normals
    let reflection = vel - 2.0*vel.dot(n) * n;
    return reflection;
}

// Assuming perfect reflection of components direction
fn reflection_box(pos: Vec3, vel: Vec3) -> Vec3
{
    let mut v = vel;
    if (pos.x < PHYS_EPSILON || pos.x > (BOX_SIZE - PHYS_EPSILON)) {v.x = -v.x;}
    if (pos.y < PHYS_EPSILON || pos.y > (BOX_SIZE - PHYS_EPSILON)) {v.y = -v.y;}
    if (pos.z < PHYS_EPSILON || pos.z > (BOX_SIZE - PHYS_EPSILON)) {v.z = -v.z;}

    return v;
}

/***
*   Collision computations
***/

// Solving the intersection time of the trajectory to the sphere via the equation for t
//      |P + tV|^2 = r^2
// where: 
//      P is the relative position vector towards the sphere center, 
//      V is the velocity vector (assumed to be normalized for easier math)
//      r is the sphere radius scalar
fn sphere_intersection_time(pos: Vec3, vel: Vec3) -> Option<f32>
{
    // Relative position offset towards the sphere center
    let center_offset: Vec3 = pos - SPHERE_CENTER;

    // Calculate discriminant to find solutions
    let b: f32 = 2.0*center_offset.dot(vel);
    let c: f32 = center_offset.length_squared() - SPHERE_RADIUS*SPHERE_RADIUS;
    let discriminant: f32 = b*b - 4.0*c;

    // Solution existence checks first (i.e. does it actually hit the sphere?)
    if (discriminant < 0.0)     { return None;}
    
    // Finding the 2 roots and return the correct time
    let (t1, t2) = (0.5*(-b - discriminant.sqrt()), 0.5*(-b + discriminant.sqrt()));
    if      (t1 > PHYS_EPSILON) { return Some(t1);}
    else if (t2 > PHYS_EPSILON) { return Some(t2);}

    return None;
}

fn box_intersection_time() -> Option<f32>
{
    unimplemented!();
}

pub fn collision(pos: Vec3, vel: Vec3) -> Option<Vec3>
{
    unimplemented!();
}
