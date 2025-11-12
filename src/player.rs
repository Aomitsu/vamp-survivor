use hecs::World;
use macroquad::prelude::*;

use crate::components::{Player, Speed, Velocity};

pub fn player_input_system(world: &mut World){
    let mut player_vel = Vec2::ZERO;
    if macroquad::input::is_key_down(KeyCode::W) { player_vel.y -= 1. }
    if macroquad::input::is_key_down(KeyCode::S) { player_vel.y += 1. }
    if macroquad::input::is_key_down(KeyCode::A) { player_vel.x -= 1. }
    if macroquad::input::is_key_down(KeyCode::D) { player_vel.x += 1. }

    // Apply to player
    for (_id, (vel, speed)) in world.query_mut::<(&mut Velocity, &Speed)>().with::<&Player>() {
        vel.0 = player_vel.normalize_or_zero() * speed.0;
    }
}
