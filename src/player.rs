use hecs::World;
use macroquad::prelude::*;

use crate::{components::{Player, Speed}, physic::{PhysicsResources, RigidBodyHandleComponent}};

pub fn player_input_system(world: &mut World, physics: &mut PhysicsResources){
    // Query for the player entity's rigid body handle.
    for (_id, (rigibody_handle, _player, speed)) in world.query_mut::<(&RigidBodyHandleComponent, &Player, &Speed)>() {
        // Get the rigid body from the physics world using the handle.
        if let Some(body) = physics.rigid_body_set.get_mut(rigibody_handle.0) {
            let mut move_direction = vec2(0.0, 0.0);

            if is_key_down(KeyCode::W) {
                move_direction.y -= 1.0;
            }
            if is_key_down(KeyCode::S){
                move_direction.y += 1.0;
            }
            if is_key_down(KeyCode::A) {
                move_direction.x -= 1.0;
            }
            if is_key_down(KeyCode::D) {
                move_direction.x += 1.0;
            }
            
            // Set the linear velocity. Normalizing ensures consistent speed in all directions.
            let desired_velocity = move_direction.normalize_or_zero() * speed.0;
            body.set_linvel([desired_velocity.x, desired_velocity.y].into(), true);
        }
    } 
}
