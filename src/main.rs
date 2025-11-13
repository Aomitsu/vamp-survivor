use std::default;

use hecs::World;
use macroquad::prelude::*;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::{asset_server::AssetServer, components::{Player, Speed, Sprite, Text, Transform}, debug::{DebugData, debug_draw, debug_infos_system}, enemy::{EnemySpawner, enemy_ai_system, enemy_spawner_system}, physic::{collision_register, physics_step_system, setup_physics, sync_physics_world, sync_transforms}, player::{detect_player_dead, player_input_system, spawn_player}};

use crate::debug::{debug_draw_colliders_system, DebugLines};
mod debug;

mod components;
mod player;
mod enemy;
mod physic;
mod asset_server;
fn window_conf() -> Conf {
    Conf {
        window_title: "vamp-survivor".to_owned(),
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::init();

    let mut world = World::new();
    let mut physics_ressources = setup_physics();
    let mut asset_server = AssetServer::new();
    let mut enemy_spawner = EnemySpawner::default();

    if cfg!(debug_assertions) { // Debug only
        // Entity for debug lines
        world.spawn((DebugLines(Vec::new()),));
        world.spawn((DebugData::new(),));

    }

    // Load initial assets
    asset_server.prime_assets().await;

    spawn_player(&mut world, &asset_server);

    loop {
        clear_background(GRAY);
        // Update physics
        sync_physics_world(&mut world, &mut physics_ressources);
        collision_register(&mut world, &mut physics_ressources);

        // Do things with entities
        player_input_system(&mut world, &mut physics_ressources);
        enemy_spawner_system(&mut world, &asset_server, &mut enemy_spawner);
        enemy_ai_system(&mut world, &mut physics_ressources);

        detect_player_dead(&mut world);

        // Physics tick related
        physics_step_system(&mut physics_ressources);
        sync_transforms(&mut world, &mut physics_ressources);

        if cfg!(debug_assertions) { // Debug only
            // Dessine les boîtes de collision pour le débogage
            debug_draw_colliders_system(&mut world, &physics_ressources);
            debug_infos_system(&mut world);
        }

        draw_world(&mut world);
        // Send frame
        next_frame().await
    }
}

fn draw_world(world: &mut World) {
    let zoom_level = 0.0025;
    set_camera(&Camera2D {
        zoom: vec2(zoom_level, zoom_level * (screen_width() / screen_height())),
        //target: player.position_centred(),
        ..Default::default()
    });


    for(_id, (pos, sprite)) in world.query_mut::<(&Transform, &Sprite)>(){
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

    for(_id, (pos, text)) in world.query_mut::<(&Transform, &Text)>(){
        draw_text_ex(text.text.as_str(), pos.0.x, pos.0.y, TextParams { 
            color: text.color,
            ..Default::default()
         });
    }


    if cfg!(debug_assertions) {
        debug_draw(world);
    }
}
