use billiards_logic::physics::*;
use billiards_logic::tangent::*;
use glam::DVec3;
use glam::Vec3;


#[test]
fn collision_remains_in_box() {
    let pos = Vec3::new(0.8, 0.8, 0.8);
    let vel = Vec3::new(1.0, 0.7, 0.3);
    let (new_pos, _, _, _) = collision(pos, vel).unwrap();
    
    assert!(new_pos.x >= 0.0 && new_pos.x <= BOX_SIZE);
    assert!(new_pos.y >= 0.0 && new_pos.y <= BOX_SIZE);
    assert!(new_pos.z >= 0.0 && new_pos.z <= BOX_SIZE);
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