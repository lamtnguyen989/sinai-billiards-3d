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
    fn dot(self, other: Self) -> f64 {return self.dq.dot(other.dq) + self.dp.dot(other.dp);}

    // Norm
    fn norm(self) -> f64 {return self.dot(self).sqrt();}
}

/* Ergodic Statistics */
#[derive(Default, Clone)]
pub struct ErgodicStats
{
    pub lyapunov_spectra:   [f64; NUM_TANGENTS],
    pub ks_entropy:         f64,
}