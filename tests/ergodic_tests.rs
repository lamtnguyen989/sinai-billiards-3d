use billiards_logic::ergodic::*;
use billiards_logic::tangent::{NUM_TANGENTS};

#[test]
fn wiki_kaplan_yorke() {
    // Based on Wikipedia: https://en.wikipedia.org/wiki/Kaplan%E2%80%93Yorke_conjecture
    let MARGIN_OF_ERROR: f64 = 1e-2;    // This is due to Wiki page only gives up to 2 decimals data

    // Henon map
    let henon_spectra: [f64; 2] = [0.603, -2.34];
    let henonStats: ErgodicStatistics<2> = ErgodicStatistics::new(&henon_spectra);
    let expected_henon_ky_dim = 1.26;
    let computed_henon_ky_dim = henonStats.get_ky_dim();
    assert!((computed_henon_ky_dim - expected_henon_ky_dim).abs() <= MARGIN_OF_ERROR);
    assert!((henonStats.get_lyapunov_time() - 1.0/henon_spectra[0]) <= MARGIN_OF_ERROR);
    assert_eq!(henonStats.get_ks_entropy(), 0.603);

    // Lorenz system
    let lorenz_spectra: [f64; 3] = [-32.4, 2.16, 0.0];
    let lorenzStats: ErgodicStatistics<3> = ErgodicStatistics::new(&lorenz_spectra);
    let expected_lorenz_ky_dim = 2.07;
    let computed_lorenz_ky_dim = lorenzStats.get_ky_dim();
    let sorted_lorenz_spectra: [f64; 3] = lorenzStats.get_lyapunov_spectra();
    assert!((computed_lorenz_ky_dim - expected_lorenz_ky_dim).abs() <= MARGIN_OF_ERROR);
    assert_eq!(sorted_lorenz_spectra, [2.16, 0.0, -32.4]);
    assert!((lorenzStats.get_lyapunov_time() - 1.0/sorted_lorenz_spectra[0]) <= MARGIN_OF_ERROR);
    assert_eq!(lorenzStats.get_ks_entropy(), sorted_lorenz_spectra[0]);
}

// Further testing based on: https://digitalcommons.georgiasouthern.edu/cgi/viewcontent.cgi?article=3478&context=etd
#[test]
fn coupled_lorenz_test()
{
    // Setup
    let MARGIN_OF_ERROR: f64 = 1e-10;
    let coupled_lorenz_spectra: [f64; 6] = [1.1062, 0.84536, -0.012153, -0.013101, -18.366, -19.051];
    let shuffled_spectra: [f64; 6] = [-18.366, 0.84536, -0.013101, -0.012153, -19.051, 1.1062];
    let stats = ErgodicStats::new(&shuffled_spectra);
    
    // Spectra processing test
    let spectra = stats.get_lyapunov_spectra();
    assert_eq!(spectra, coupled_lorenz_spectra);

    // Lyapunov time check
    assert_eq!(1.0/spectra[0], stats.get_lyapunov_time());

    // Kaplan-Yorke dimension test
    let expected_ky_dim = 4.0 + (coupled_lorenz_spectra[0..4].iter().sum::<f64>() / f64::abs(coupled_lorenz_spectra[4]));
    let computed_ky_dim = stats.get_ky_dim();
    assert!((computed_ky_dim - expected_ky_dim).abs() < MARGIN_OF_ERROR);

    // Metric entropy test
    let expected_ks_entropy: f64 = coupled_lorenz_spectra[0..2].iter().sum();
    assert!((stats.get_ks_entropy() - expected_ks_entropy) < MARGIN_OF_ERROR);
}

#[test]
// Testing all zeros stats states for rendering initializing
fn empty_stats() {
    // No formal test, just need to make sure that we can initialize empty stats
    let spectra: [f64; NUM_TANGENTS] = [0.0; NUM_TANGENTS];
    let empty_stats = ErgodicStatistics::new(&spectra);
}