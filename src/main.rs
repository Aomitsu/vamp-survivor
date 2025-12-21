use std::default;

use hecs::World;
use macroquad::prelude::*;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::{asset_server::AssetServer, components::{Player, Speed, Sprite, Text, Transform}, debug::{DebugData, debug_draw, debug_infos_system}, enemy::{EnemySpawner, enemy_ai_system, enemy_spawner_system}, physic::{collision_register, physics_step_system, setup_physics, sync_physics_world, sync_transforms}, player::{detect_player_dead, player_input_system, spawn_player}, render::draw_world};


use crate::debug::{debug_draw_colliders_system, DebugLines};
mod debug;

mod components;
mod player;
mod enemy;
mod physic;
mod asset_server;
mod render;

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

    asset_server.load_assets(&[
        "assets/player.png",
        "assets/enemy.png"
    ]).await;

    spawn_player(&mut world);

    loop {
        clear_background(GRAY);
        // Update physics
        sync_physics_world(&mut world, &mut physics_ressources);
        collision_register(&mut world, &mut physics_ressources);

        // Do things with entities
        player_input_system(&mut world, &mut physics_ressources);
        enemy_spawner_system(&mut world, &mut enemy_spawner);
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

        draw_world(&mut world, &asset_server);
        // Send frame
        next_frame().await
    }
}
