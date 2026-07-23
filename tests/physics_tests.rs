use billiards_logic::physics::*;
use billiards_logic::tangent::*;
use billiards_logic::lyapunov::*;
use billiards_logic::config::{PhysicsConfig};

use glam::{DVec3, Vec3};
use nalgebra::{SMatrix};
use rand::{
    SeedableRng, 
    rngs::StdRng
};

// Default physics config for quick test refactoring
fn test_config(box_sz: f32, radius: f32) -> PhysicsConfig {return PhysicsConfig::new(box_sz, radius);}
fn test_config_default() -> PhysicsConfig {return PhysicsConfig::new(1.0, 0.25);}

#[test]
fn collision_remains_in_box() {
    let config = test_config_default();
    let pos = Vec3::new(0.8, 0.8, 0.8);
    let vel = Vec3::new(1.0, 0.7, 0.3);
    let (new_pos, _, _, _) = collision(pos, vel, config).unwrap();
    
    assert!(new_pos.x >= 0.0 && new_pos.x <= config.box_size());
    assert!(new_pos.y >= 0.0 && new_pos.y <= config.box_size());
    assert!(new_pos.z >= 0.0 && new_pos.z <= config.box_size());
}

#[test]
fn test_phase_tangent_creation() {
    assert_eq!(NUM_TANGENTS, 6);
    let arr_val: [f64; NUM_TANGENTS] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let pt = TangentPhaseVector::from_array(arr_val);

    assert_eq!(pt.get_position_tangent(), DVec3::new(arr_val[0], arr_val[1], arr_val[2]));
    assert_eq!(pt.get_momentum_tangent(), DVec3::new(arr_val[3], arr_val[4], arr_val[5]));
    assert_eq!(pt.as_array().len(), NUM_TANGENTS);
    assert_eq!(pt.as_array(), arr_val);
}

#[test]
fn test_phase_tangent_arithmetic() {
    // Creating phase points
    let a1: [f64; NUM_TANGENTS] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let a2: [f64; NUM_TANGENTS] = [7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
    let pt1 = TangentPhaseVector::from_array(a1);
    let pt2 = TangentPhaseVector::from_array(a2);
    let r: f64 = 1.5;

    // Addition tests
    assert_eq!((pt1 + pt2).get_position_tangent(), DVec3::new(a1[0]+a2[0], a1[1]+a2[1], a1[2]+a2[2]));
    assert_eq!((pt1 + pt2).get_momentum_tangent(), DVec3::new(a1[3]+a2[3], a1[4]+a2[4], a1[5]+a2[5]));

    // Subtraction tests
    assert_eq!((pt1 - pt2).get_position_tangent(), DVec3::new(a1[0]-a2[0], a1[1]-a2[1], a1[2]-a2[2]));
    assert_eq!((pt1 - pt2).get_momentum_tangent(), DVec3::new(a1[3]-a2[3], a1[4]-a2[4], a1[5]-a2[5]));

    // Scalar multiplication tests
    assert_eq!((pt1*r).get_position_tangent(),  DVec3::new(a1[0]*r, a1[1]*r, a1[2]*r));
    assert_eq!((pt1*r).get_momentum_tangent(),  r*DVec3::new(a1[3], a1[4], a1[5]));
    assert_eq!((r*pt2).get_position_tangent(),  DVec3::new(a2[0]*r, a2[1]*r, a2[2]*r));
    assert_eq!((r*pt2).get_momentum_tangent(),  r*DVec3::new(a2[3], a2[4], a2[5]));
}


#[test]
fn test_arnold_cat_lya_spectra() {
    // Practical parameters
    let margin_of_error: f64 = 1e-5;
    let simulation_steps = 1e5 as usize;

    // Setting spectra object and base map matrix
    let arnold_cat_map_column_slice: [f64; 4] = [2.0, 1.0, 1.0, 1.0];
    let cat_map_matrix = SMatrix::from_column_slice(&arnold_cat_map_column_slice);
    let mut arnold_cat_spectra = LyapunovSpectra::<2>::new();

    // Iteratively simulate for high steps 
    for step in 1..simulation_steps {
        // Compute iteration's frame and update
        let new_frame: SMatrix<f64, 2, 2> = cat_map_matrix * arnold_cat_spectra.get_frame();
        arnold_cat_spectra.frame_from_slice(new_frame.as_slice(), FrameLayout::ColumnMajor);

        // Update in "discrete" time
        arnold_cat_spectra.compute_from_frame(1.0, step as f64);
    }

    // Test
    let computed_spectra = arnold_cat_spectra.get_spectrum();
    let expected_spectra: [f64; 2] = [f64::ln(0.5* (3.0 + f64::sqrt(5.0))), f64::ln(0.5* (3.0 - f64::sqrt(5.0)))];
    assert!((expected_spectra[0] - computed_spectra[0]).abs() < margin_of_error, "Expected: {}, Actual: {}", expected_spectra[0], computed_spectra[0]);
    assert!((expected_spectra[1] - computed_spectra[1]).abs() < margin_of_error, "Expected: {}, Actual: {}", expected_spectra[1], computed_spectra[1]);
}

#[test]
fn test_random_trajectory_stays_in_box() {
    // Setup random trajectory
    let config = test_config_default();
    let mut rng = StdRng::seed_from_u64(420);
    let mut traj = random_trajectory(&mut rng, [1.0, 0.0, 0.0, 1.0], config);
    let test_epsilon: f32 = 1e-6;    // Stronger than PHYS_EPSILON
    let steps = 1000;

    // Run trajectory and check
    for k in 0..steps {
        traj.update(1).unwrap();
        let p = traj.current_pos();
        let v = traj.current_vel();
        assert!(
            p.x >= -test_epsilon && p.x <= (config.box_size() + test_epsilon) &&
            p.y >= -test_epsilon && p.y <= (config.box_size() + test_epsilon) &&
            p.z >= -test_epsilon && p.z <= (config.box_size() + test_epsilon),
            "Escaped box at step {}: pos={:?} vel={:?}", k, p, v
        );
    }
}


#[test]
fn test_qualitative_trajectory_lya_spectra_properties() {
    // Setup random trajectory
    let config = test_config(1.25, 0.5);
    let mut rng = StdRng::seed_from_u64(69);
    let mut traj = random_trajectory(&mut rng, [1.0, 0.0, 0.0, 1.0], config);

    // Update trajectory
    let steps = 20000;  // Recall Lyapunov exponents is an infinite limit so larger iteration steps are prefered
    for _k in 0..steps {traj.update(1).unwrap();}

    // Testing pairing symmetry
    let margin_of_error = 5e-4; // Should be enough for rendering purposes
    let spectra = traj.curr_lya_spectra();
    for k in 0..NUM_TANGENTS/2 {
        let pair_sum: f64 = spectra[k] + spectra[NUM_TANGENTS-1-k];
        assert!(pair_sum.abs() < margin_of_error,
                "Pairing broken at index: {} with {} + {} = {}", k, spectra[k], spectra[NUM_TANGENTS-1-k], pair_sum);
        
    }

    // Testing spectra sum should converge to zero
    let spectra_sum: f64 = traj.curr_lya_spectra().iter().sum();
    assert!(spectra_sum.abs() < margin_of_error, "Lyapunov sum = {}", spectra_sum);

    // Test that it is at least chaotic
    assert!(spectra[0] > 0.0, "This ain't chaotic! Leading exponent: {}", spectra[0]);

    // Test the middle exponents should be close to zero
    assert!(spectra[2].abs() < margin_of_error && spectra[3].abs() < margin_of_error, 
            "Middle exponents not close to zero! Actual values: {} and {}", spectra[2], spectra[3]);
}
