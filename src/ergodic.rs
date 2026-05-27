

use glam::Vec3;
use glam::DVec3;
use crate::physics::*;

/* Constants */

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
// Also this is most likely zero for this billiards model although it is worth it to give it a proper treatment
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
