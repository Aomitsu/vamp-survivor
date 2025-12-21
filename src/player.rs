use hecs::{Entity, World};
use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{asset_server::{self, AssetServer}, components::*, physic::{self, CollideWith, PhysicsResources, RigidBodyHandleComponent}};

pub fn spawn_player(world: &mut World){
    let player_body = RigidBodyBuilder::dynamic().lock_rotations().build();
    let player_collider = ColliderBuilder::cuboid(16., 16.)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .translation([32./2., 32./2.].into())
        .build();

    // Get texture from AssetServer
    world.spawn((
        Player,
        Transform(vec2(0.0, 0.0)),
        Speed(200.),
        Sprite { 
            asset_id: asset_server::assets::player(),
            scale: 1.0 
        },
        player_body,
        player_collider,
        Health { actual: 100., max: 100. }
    ));
}

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


pub fn detect_player_dead(world: &mut World) {
    // Debug show all entity in CollideWith
    for (_id, collidewith) in world.query::<&CollideWith>().with::<&Player>().iter() {
        log::debug!("Entities who collide with : {:?}", collidewith.0);
    }


    for (id, health) in world.query::<&Health>().with::<&Player>().iter() {
        if health.actual <= 0.0 {
            println!("Player has died! Game Over.");
        }
    }
}