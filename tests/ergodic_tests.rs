use billiards_logic::ergodic::*;
use glam::Vec3;
use glam::DVec3;


#[test]
fn wiki_ky_dim(){
    // Based on Wikipedia: https://en.wikipedia.org/wiki/Kaplan%E2%80%93Yorke_conjecture
    let MARGIN_OF_ERROR: f64 = 1e-2;    // This is due to Wiki page only gives up to 2 decimals data

    // Henon map
    let henon_spectra: [f64; 2] = [0.603, -2.34];
    let computed_henon_ky_dim = kaplan_yorke_dim(&henon_spectra);
    let expected_henon_ky_dim = 1.26;
    assert!((computed_henon_ky_dim - expected_henon_ky_dim).abs() <= MARGIN_OF_ERROR);

    // Lorenz system
    let lorenz_spectra: [f64; 3] = [2.16, 0.0, -32.4];
    let computed_lorenz_ky_dim = kaplan_yorke_dim(&lorenz_spectra);
    let expected_lorenz_ky_dim = 2.07;
    assert!((computed_lorenz_ky_dim - expected_lorenz_ky_dim).abs() <= MARGIN_OF_ERROR);
}

