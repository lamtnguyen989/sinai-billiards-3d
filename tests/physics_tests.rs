use billiards_logic::physics::*;
use glam::Vec3;


#[test]
fn collision_remains_in_box(){
    let pos = Vec3::new(0.8, 0.8, 0.8);
    let vel = Vec3::new(1.0, 0.7, 0.3);
    let (new_pos, _) = collision(pos, vel).unwrap();
    
    assert!(new_pos.x >= 0.0 && new_pos.x <= BOX_SIZE);
    assert!(new_pos.y >= 0.0 && new_pos.y <= BOX_SIZE);
    assert!(new_pos.z >= 0.0 && new_pos.z <= BOX_SIZE);
}
