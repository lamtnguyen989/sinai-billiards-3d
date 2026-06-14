use crate::tangent::*;
use crate::physics::*;


/* Ergodic Statistics */
// Need to make generic version of Ergodic Stat based on Lyapunov spectra length purely for testing
#[derive(Clone)]
pub struct ErgodicStatistics<const N: usize>
{
    lyapunov_spectra:   [f64; N],
    ks_entropy:         f64,
    kaplan_yorke_dim:   f64,
}

impl<const N: usize> ErgodicStatistics<N>
{
    pub fn new(lya_spectra: &[f64; N]) -> Self {
        // Make a mutable copy of the specta and sort it
        let mut spectra = *lya_spectra;
        spectra.sort_by(|a,b| {b.partial_cmp(a).unwrap()});

        // Compute statistics
        let ks_entropy = spectra.iter().map(|&x| {x.max(0.0)}).sum();
        let ky_dim = kaplan_yorke_dim(&spectra);

        return Self {
            lyapunov_spectra:   spectra,
            ks_entropy:         ks_entropy,
            kaplan_yorke_dim:   ky_dim
        };
    }

    // Getters
    pub fn get_lyapunov_spectra(&self) -> [f64; N] {return self.lyapunov_spectra;}
    pub fn get_lyapunov_time(&self) -> f64 {if self.lyapunov_spectra[0] == 0.0 {return 0.0;} else {return 1.0/self.lyapunov_spectra[0];}}
    pub fn get_ks_entropy(&self) -> f64 {return self.ks_entropy;}
    pub fn get_ky_dim(&self) -> f64 {return self.kaplan_yorke_dim;}
}

// The main type we will be using for ergodic computations of the billiards
pub type ErgodicStats = ErgodicStatistics<NUM_TANGENTS>;
impl ErgodicStats
{
    // Compute ergodic quantities from the trajectory
    pub fn compute_from_trajectory(traj: &Trajectory) -> Self {

        // Get and sort the Lyapunov spectra
        let mut spectra: [f64; NUM_TANGENTS] = traj.curr_lya_spectra();
        spectra.sort_by(|a,b| {b.partial_cmp(a).unwrap()});

        // Compute statistics
        let ks_entropy = spectra.iter().map(|&x| {x.max(0.0)}).sum();
        let ky_dim = kaplan_yorke_dim(&spectra);

        return Self {
            lyapunov_spectra:   spectra,
            ks_entropy:         ks_entropy,
            kaplan_yorke_dim:   ky_dim
        }
    }
}

// Kaplan-Yorke dimension
// IMPORTANT: This assume we have sorted the spectra
// Also this is most likely integer-valued for this billiards model although it is worth it to give it a proper treatment
fn kaplan_yorke_dim(lya_spectra: &[f64]) -> f64
{
    let mut sum: f64 = 0.0;
    let mut positive_count: usize = 0;

    for (k, &lya_exp) in lya_spectra.iter().enumerate() {
        // Find the point in which sum turns negative   
        if sum + lya_exp < 0.0 {
            break;
        }
        // Increment
        sum += lya_exp;
        positive_count = k + 1;
    }

    // Return early guard for initializing case (spectra = [0.0; N])
    if positive_count >= lya_spectra.len() {
        return lya_spectra.len() as f64;
    }

    // All positive sum has no trailing dimension computation
    let neg_point_abs = lya_spectra[positive_count].abs();
    if neg_point_abs < 1e-16 {
        return lya_spectra.len() as f64;
    }
    // Follow the formula from here
    return positive_count as f64 + sum/neg_point_abs;
}
