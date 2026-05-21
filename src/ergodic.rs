use crate::physics::*;

use glam::Vec3;
use glam::DVec3;

/* Constants */
pub const NUM_TANGENTS: usize = 6;

/* Tangent vector */
#[derive(Clone, Copy, Default)]
struct TangentVector
{
    dp: DVec3,  // Position tangent
    dq: DVec3,  // Momentum tangent
}

impl TangentVector
{
    // Dot product
    fn dot(self, other: Self) -> f64 {
        return self.dq.dot(other.dq) + self.dp.dot(other.dp);
    }

    // Norm
    fn norm(self) -> f64 {return self.dot(self).sqrt();}
}

/* Billiards trajectory */
#[derive(Clone)]
pub struct Trajectory
{
    pub positions:                  Vec<glam::Vec3>,
    pub velocities:                 Vec<glam::Vec3>,
    pub current_lyapunov_spectra:   [f64; NUM_TANGENTS],
}

impl Trajectory
{

    // Getters for current position and velocities
    pub fn current_pos(&self) -> Vec3 {return *self.positions.last().unwrap();}
    pub fn current_vel(&self) -> Vec3 {return *self.velocities.last().unwrap();}
}

/* Ergodic Statistics */
#[derive(Default, Clone)]
pub struct ErgodicStats
{
    pub lyapunov_spectra:   [f64; NUM_TANGENTS],
    pub ks_entropy:         f64,
}