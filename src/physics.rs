use rand::{Rng, RngExt};
use glam::{Vec3, DVec3};
use nalgebra::{Matrix6};

use crate::tangent::{NUM_TANGENTS, TangentPhaseVector};
use crate::lyapunov::{LyapunovSpectra, FrameLayout};

/// Note the model assume unit mass so velocities and momenta are interchangable

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
    if pos.x < PHYS_EPSILON || pos.x > (BOX_SIZE - PHYS_EPSILON) {v.x = -v.x;}
    if pos.y < PHYS_EPSILON || pos.y > (BOX_SIZE - PHYS_EPSILON) {v.y = -v.y;}
    if pos.z < PHYS_EPSILON || pos.z > (BOX_SIZE - PHYS_EPSILON) {v.z = -v.z;}

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
    if discriminant < 0.0       { return None;}
    
    // Finding the 2 roots and return the correct time
    let (t1, t2) = (0.5*(-b - discriminant.sqrt()), 0.5*(-b + discriminant.sqrt()));
    if      t1 > PHYS_EPSILON { return Some(t1);}
    else if t2 > PHYS_EPSILON { return Some(t2);}
    else                      { return None;}
    
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

    for k in 0..3 {
        let v = vel[k];
        let p = pos[k];

        if v.abs() < PHYS_EPSILON {
            if p < 0.0 || p > BOX_SIZE { return None;}
            continue;
        }
        let (t0, t1) : (f32, f32) = (-p/v, (BOX_SIZE - p)/v);
        let (lo, hi) =  if t0 < t1 { (t0, t1) } else { (t1, t0) };
        
        t_min = f32::max(t_min, lo);
        t_max = f32::min(t_max, hi);
        if t_min > t_max { return None; }
    }

    if      t_min > PHYS_EPSILON  { return Some(t_min);}
    else if t_max > PHYS_EPSILON  { return Some(t_max);}
    else                          { return None;}
}

pub fn collision(pos: Vec3, vel: Vec3) -> Option<(Vec3, Vec3, f32, bool)>
{
    // Normalize the velocity as the intersections depends on it
    let v = vel.normalize();

    // Compute the intersection times
    let t_sph : Option<f32> = sphere_intersection_time(pos, v);
    let t_box : Option<f32> = box_intersection_time(pos, v);

    // Process the intersection times
    let (t, hit_sphere) : (f32, bool) = match(t_sph, t_box) {
        (Some(ts), Some(tb)) if ts < tb => (ts, true),
        (_, Some(tb))                   => (tb, false),
        (Some(ts), _)                   => (ts, true),
        _                               => return None
    }; 
    if t < PHYS_EPSILON { return None;}
    
    // Compute the new position and velocity
    let new_pos : Vec3 = pos + t*v;
    let new_vel : Vec3 = if hit_sphere {reflection_sphere(new_pos, v)} 
                        else {reflection_box(new_pos, v)};

    return Some((new_pos, new_vel, t, hit_sphere));
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
fn phase_tangent_wall_reflect(tpv: TangentPhaseVector, n: DVec3) -> TangentPhaseVector 
{
    let tpv_position: DVec3 = tpv.get_position_tangent();
    let tpv_momentum: DVec3 = tpv.get_momentum_tangent();

    let tangent_pos_reflection: DVec3 = tpv_position - (2.0 * tpv_position.dot(n))*n;
    let tangent_mom_reflection: DVec3 = tpv_momentum - (2.0 * tpv_momentum.dot(n))*n;
    return TangentPhaseVector::new(tangent_pos_reflection, tangent_mom_reflection);
}

// Sphere reflection perturbation
// TODO: Explain the mathematical mess of the derivation
fn phase_tangent_sphere_reflect(tpv: TangentPhaseVector, incoming_momentum: DVec3, n: DVec3, r: f64) -> TangentPhaseVector 
{
    let tpv_position: DVec3 = tpv.get_position_tangent();
    let tpv_momentum: DVec3 = tpv.get_momentum_tangent();

    let pos_reflection: DVec3 = tpv_position - (2.0 * tpv_position.dot(n))*n;  
    let mom_reflection: DVec3 = tpv_momentum - (2.0 * tpv_momentum.dot(n))*n;

    let sphere_correction:DVec3 = incoming_momentum.dot(n) * tpv_position 
                                - tpv_position.dot(n) * incoming_momentum 
                                + incoming_momentum.dot(tpv_position) * n
                                - (incoming_momentum.dot(incoming_momentum) / incoming_momentum.dot(n)) * (tpv_position.dot(n)) * n;

    return TangentPhaseVector::new(pos_reflection, mom_reflection - 2.0/r*sphere_correction);
}


/*** 
*   Billiards trajectory 
***/
#[derive(Debug)]
pub enum TrajectoryError {
    NoCollision,
    UnknownWallNormal
}

#[derive(Clone)]
pub struct Trajectory
{
    // Actual physics and math fields
    positions:              Vec<glam::Vec3>,
    velocities:             Vec<glam::Vec3>,
    lyapunov_spectra:       TrajectoryPhaseLyapunovSpectra,
    collision_count:        usize,
    distance_travelled:     f64,    // This is also the total simulation time due to |x| = t*|v| and we are using unit velocity |v| = 1

    // Extra rendering data
    pub color:              [f32; 4],   // RGBA values
}

impl Trajectory
{
    // Constructor
    pub fn new(pos: Vec3, vel: Vec3, color: [f32; 4]) -> Self {
        return Self {
            positions:              vec![pos],
            velocities:             vec![vel.normalize()],
            lyapunov_spectra:       TrajectoryPhaseLyapunovSpectra::new(),
            collision_count:        0,
            distance_travelled:     0.0,
            color:                  color
        }
    }

    // Getters
    pub fn current_pos(&self) -> Vec3 {return *self.positions.last().unwrap();}
    pub fn current_vel(&self) -> Vec3 {return *self.velocities.last().unwrap();}
    pub fn curr_lya_spectra(&self) -> [f64; NUM_TANGENTS] {return self.lyapunov_spectra.get_spectrum();}
    pub fn get_collision_count(&self) -> usize {return self.collision_count;}
    pub fn get_mean_free_path(&self) -> f64 {return self.distance_travelled / self.collision_count as f64;}

    // Update trajectory
    pub fn update(&mut self, max_history: usize) -> Result<(), TrajectoryError> {
        // Extracting current position and velocity
        let pos = self.current_pos();
        let vel = self.current_vel();

        // Compute the next collision phase point
        let (new_pos, new_vel, t, hit_sphere) = collision(pos, vel).ok_or(TrajectoryError::NoCollision)?;

        // Compute the normal vectors
        let n_wall: DVec3 = wall_normal(new_pos).ok_or(TrajectoryError::UnknownWallNormal)?;
        let n_sphere: DVec3 = (new_pos - SPHERE_CENTER).as_dvec3() / SPHERE_RADIUS as f64;

        // Update 
        let incoming_vel = vel.as_dvec3();
        if self.positions.len() >= max_history  {self.positions.remove(0);}
        if self.velocities.len() >= max_history {self.velocities.remove(0);}

        self.positions.push(new_pos);
        self.velocities.push(new_vel);
        self.collision_count += 1;
        self.distance_travelled += t as f64;
        self.lyapunov_spectra.update_spectrum(t as f64, hit_sphere, incoming_vel, n_wall, n_sphere, self.distance_travelled);

        Ok(())
    }
}

// Find nearest wall's outward normal to the trajectory
fn wall_normal(pos: Vec3) -> Option<DVec3> {
    let wall_distances: [f32; 3] = std::array::from_fn(|k| {pos[k].min(BOX_SIZE - pos[k])});
    match wall_distances.iter().enumerate()
                        .min_by(|(_, a), (_, b)| {a.partial_cmp(b).unwrap()})
                        .unwrap().0 
    {
        0 => Some(DVec3::X),
        1 => Some(DVec3::Y),
        2 => Some(DVec3::Z),
        _ => None
    }
}

/***
*   Specialized handling of Lyapunov spectra for the trajectory phase
***/
type TrajectoryPhaseLyapunovSpectra = LyapunovSpectra<NUM_TANGENTS>;
impl TrajectoryPhaseLyapunovSpectra
{
    // Compute Lyapunov spectrum after a collision
    pub fn update_spectrum(&mut self, t: f64, hit_sphere: bool,
                        momentum_in: DVec3, n_wall: DVec3, n_sphere: DVec3,
                        total_time: f64) 
    {
        // Compute the phase frame both in free-flight and after collision
        compute_trajectory_phase_frame(&mut self.get_frame_mut(), |w| {phase_tangent_free_flight(w, t)});
        self.reorthorgonalize_frame();  // Re-orthogonalize for improved stability after free flight
        
        compute_trajectory_phase_frame(&mut self.get_frame_mut(), |w| {
            if hit_sphere   {phase_tangent_sphere_reflect(w, momentum_in, n_sphere, SPHERE_RADIUS as f64)}
            else            {phase_tangent_wall_reflect(w, n_wall)}
        });

        // Update the spectra and re-orthonormalize the frame
        self.compute_from_frame(t, total_time);
    }
}


fn compute_trajectory_phase_frame(frame: &mut Matrix6<f64>, compute_type: impl Fn(TangentPhaseVector) -> TangentPhaseVector) -> ()
{
    // Compute the phase frame column-by-column
    for col in 0..NUM_TANGENTS {
        let curr_column_values: [f64; NUM_TANGENTS] = std::array::from_fn(|k| frame[(k, col)]);
        let updated_column_values: [f64; NUM_TANGENTS] = compute_type(TangentPhaseVector::from_array(curr_column_values)).as_array();
        for k in 0..NUM_TANGENTS {frame[(k, col)] = updated_column_values[k];}
    }
}


/***
*   Random trajectory spawner for development
***/
pub fn random_trajectory<R: Rng>(rng: &mut R, color: [f32; 4]) -> Trajectory
{
    // Random position outside the sphere (not really, but this is for testing purposes)
    let p = Vec3::new(
        0.8*BOX_SIZE * rng.random::<f32>() + BOX_SIZE * 0.1,
        0.8*BOX_SIZE * rng.random::<f32>() + BOX_SIZE * 0.1,
        0.8*BOX_SIZE * rng.random::<f32>() + BOX_SIZE * 0.1,
    );

    // Random unit velocity vector (could leave it random here due to constructor normalize no matter what, but again testing purposes)
    let random_unit_vel = Vec3::new(    
        rng.random_range(-1.0..1.0),
        rng.random_range(-1.0..1.0),
        rng.random_range(-1.0..1.0),
    ).normalize();

    return Trajectory::new(p, random_unit_vel, color);
}