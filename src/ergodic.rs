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
    pub kaplan_yorke_dim:           f64,
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
    pub kaplan_yorke_dim:   f64,
}

// TODO: Calculate Lyapunov spectra


// Kaplan-Yorke dimension
// IMPORTANT: This assume we have sorted the spectra (not sorting here since there can be a way to shoehorn the sorting somewhere else)
fn kaplan_yorke_dim(lya_spectra: &[f64]) -> f64
{
    let mut sum: f64 = 0.0;
    let mut positive_count: usize = 0;

    for (k, &lya_exp) in lya_spectra.iter().enumerate() {
        // Find the point in which sum turns negative   
        if (sum + lya_exp < 0.0) {
            break;
        }
        // Increment
        sum += lya_exp;
        positive_count = k + 1;
    }

    // All positive sum has no trailing dimension computation
    let neg_point_abs = lya_spectra[positive_count].abs();
    if (positive_count >= lya_spectra.len() || neg_point_abs < 1e-16) {
        return lya_spectra.len() as f64;
    }
    // Follow the formula from here
    return positive_count as f64 + sum/neg_point_abs;
}