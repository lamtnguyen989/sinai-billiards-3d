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

/// Overloading operators for arithmetics ///
// Addition 
impl std::ops::Add for TangentVector 
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self {
            dp: self.dp + other.dp,
            dq: self.dq + other.dq
        }
    }
}

impl std::ops::AddAssign for TangentVector
{
    fn add_assign(&mut self, other: Self) {
        self.dp += other.dp;
        self.dq += other.dq;
    }
}

// Subtraction
impl std::ops::Sub for TangentVector
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Self {
            dp: self.dp - other.dp,
            dq: self.dq - other.dq
        }
    }
}

impl std::ops::SubAssign for TangentVector
{
    fn sub_assign(&mut self, other: Self) {
        self.dp -= other.dp;
        self.dq -= other.dq;
    }
}

// Scalar multiplication
impl std::ops::Mul<f64> for TangentVector
{
    type Output = Self;
    fn mul(self, a: f64) -> Self {
        return Self {
            dp : a * self.dp,
            dq : a * self.dq
        }
    }
}

impl std::ops::MulAssign<f64> for TangentVector
{
    fn mul_assign(&mut self, a: f64) {
        self.dp *= a;
        self.dq *= a;
    }
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