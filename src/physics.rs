use rand::Rng;
use glam::{Vec3, DVec3};
use crate::tangent::{NUM_TANGENTS, TangentPhaseVector};

/*** 
*   Geometry constants
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

// Box reflection
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
    else                        { return None;}
    
}

// Finding the trajectory's intersection time to the boundary box
// The box model is [0, L]^3 where L = BOX_SIZE
// Essentially for each dimension k in {x,y,z}, solve:
//      p_k + t_0*v_k = 0     and     p_k + t_L*v_k = 0
// to find the entry and exit time candidates for the dimensions.
// From here, just compute the range within all the direction for the time the ray being in the cube.
fn box_intersection_time(pos: Vec3, vel: Vec3) -> Option<f32>
{
    let mut t_min = PHYS_EPSILON;
    let mut t_max = 1e10_f32;

    for k in (0..3) {
        let v = vel[k];
        let p = pos[k];

        if (v.abs() < PHYS_EPSILON)
        {
            if (p < 0.0 || p > BOX_SIZE) { return None;}
            continue;
        }
        let (t0, t1) : (f32, f32) = (-p/v, (BOX_SIZE - p)/v);
        let (lo, hi) =  if t0 < t1 { (t0, t1) } else { (t1, t0) };
        
        t_min = f32::max(t_min, lo);
        t_max = f32::min(t_max, hi);
        if (t_min > t_max) { return None; }
    }

    if      (t_min > PHYS_EPSILON)  { return Some(t_min);}
    else if (t_max > PHYS_EPSILON)  { return Some(t_max);}
    else                            { return None;}
}

pub fn collision(pos: Vec3, vel: Vec3) -> Option<(Vec3, Vec3)>
{
    // Normalize the velocity as the intersections depends on it
    let v = vel.normalize();

    // Compute the intersection times
    let t_sph : Option<f32> = sphere_intersection_time(pos, v);
    let t_box : Option<f32> = box_intersection_time(pos, v);

    // Process the intersection times
    let (t, hit_sphere) : (f32, bool) = match(t_sph, t_box) {
        (Some(ts), Some(tb)) if (ts < tb)   => (ts, true),
        (_, Some(tb))                       => (tb, false),
        (Some(ts), _)                       => (ts, true),
        _                                   => return None
    }; 
    if (t < PHYS_EPSILON) { return None;}
    
    // Compute the new position and velocity
    let new_pos : Vec3 = (pos + t*v);
    let new_vel : Vec3 = if (hit_sphere) {reflection_sphere(new_pos, v)}
                        else {reflection_box(new_pos, v)};

    return Some((new_pos, new_vel));
}


/*** 
*   Phase space perturbation physics
***/
// Free flight 
fn phase_tangent_free_flight(p: TangentPhaseVector, t: f64) -> TangentPhaseVector 
{
    let p_position = p.get_position_tangent();
    let p_momentum = p.get_momentum_tangent();

    return TangentPhaseVector::new(p_position + t*p_momentum, p_momentum);
}

// Wall reflection just basically reflect both position and momentum through the normal
fn phase_tangent_wall_reflect(p: TangentPhaseVector, n: DVec3) -> TangentPhaseVector 
{
    let p_position = p.get_position_tangent();
    let p_momentum = p.get_momentum_tangent();

    let tangent_pos_reflection: DVec3 = p_position - (2.0 * p_position.dot(n))*n;
    let tangent_mom_reflection: DVec3 = p_momentum - (2.0 * p_momentum.dot(n))*n;
    return TangentPhaseVector::new(tangent_pos_reflection, tangent_mom_reflection);
}

fn phase_tangent_sphere_reflect(p: TangentPhaseVector, n: DVec3) -> TangentPhaseVector 
{
    todo!("Implement how both the position and momentum is perturbed after hitting the sphere");
}


/*** 
*   Billiards trajectory 
***/
#[derive(Clone, Default)]
pub struct Trajectory
{
    // Actual physics and math fields
    pub positions:                  Vec<glam::Vec3>,
    pub velocities:                 Vec<glam::Vec3>,
    pub current_lyapunov_spectra:   [f64; NUM_TANGENTS],
    pub kaplan_yorke_dim:           f64,

    // Extra rendering data
    pub collision_count:            usize,
    pub color:                      [f32; 4],   // RGBA values
}

impl Trajectory
{
    // Constructor
    pub fn new(pos: Vec3, vel: Vec3, color: [f32; 4]) -> Self {
        return Self {
            positions:                  vec![pos],
            velocities:                 vec![vel.normalize()],
            current_lyapunov_spectra:   [0.0; NUM_TANGENTS],
            kaplan_yorke_dim :          0.0,
            collision_count:            0,
            color:                      color
        }
    }

    // Getters
    pub fn current_pos(&self) -> Vec3 {return *self.positions.last().unwrap();}
    pub fn current_vel(&self) -> Vec3 {return *self.velocities.last().unwrap();}
    pub fn current_spectra(&self) -> &[f64; NUM_TANGENTS] {return &self.current_lyapunov_spectra;}
    pub fn current_lya_time(&self) -> f64 {return 1.0/self.current_lyapunov_spectra[0];}    // Ehh assume sorted spectra
}
