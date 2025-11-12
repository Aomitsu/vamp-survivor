use std::default;

use hecs::World;
use macroquad::prelude::*;

use crate::{components::{Player, Position, RectangleLines, Speed, Sprite, Velocity}, entity::movement_system, physic::{physics_step_system, setup_physics, sync_physics_world, sync_transforms}, player::player_input_system};

mod components;
mod player;
mod entity;
mod physic;
fn window_conf() -> Conf {
    Conf {
        window_title: "vamp-survivor".to_owned(),
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    let mut physics_ressources = setup_physics();

    let player_texture = load_texture("assets/player.png").await.unwrap();

    world.spawn((
        Player,
        Position(vec2(0.0, 0.0)),
        Velocity(Vec2::ZERO),
        Speed(10.),
        Sprite { texture: player_texture, scale: 1.0 },
    ));

    loop {
        clear_background(GRAY);
        // Update physics
        sync_physics_world(&mut world, &mut physics_ressources);

        // Do things with entities
        player_input_system(&mut world);

        // Physics tick related
        physics_step_system(&mut physics_ressources);
        sync_transforms(&mut world, &mut physics_ressources);

        draw_world(&mut world);
        // Send frame
        next_frame().await
    }
}

fn draw_world(world: &mut World) {
    for(_id, (pos, sprite)) in world.query_mut::<(&Position, &Sprite)>(){
        draw_texture_ex(
            &sprite.texture,
            pos.0.x,
            pos.0.y,
            WHITE,
            DrawTextureParams { 
                dest_size: Some(vec2(sprite.texture.width() * sprite.scale, sprite.texture.height() * sprite.scale)),
                ..Default::default()
                }
            )
    }
    for(_id, (pos, box_to_render)) in world.query_mut::<(&Position, &RectangleLines)>(){
        draw_rectangle_lines(pos.0.x, pos.0.y, box_to_render.size.x, box_to_render.size.y, box_to_render.thicness, box_to_render.color);
    }
}