use hecs::World;
use macroquad::prelude::*;

use crate::{
    asset_server::AssetServer,
    components::{
        render::{CameraZoom, MainCamera},
        transform::Transform,
    },
    debug::{DebugData, debug_infos_system},
    enemy::{EnemySpawner, detect_enemy_dead, enemy_ai_system, enemy_spawner_system},
    physic::{
        collision_register, physics_cleanup_system, physics_step_system, setup_physics,
        sync_physics_world, sync_transforms,
    },
    player::{detect_player_dead, player_input_system, spawn_player},
    projectile::detect_projectiles_collision,
    render::draw_world,
    resources::GameTick,
};

use crate::debug::{DebugLines, debug_draw_colliders_system};
mod debug;

mod asset_server;
mod components;
mod enemy;
mod physic;
mod player;
mod projectile;
mod render;
mod resources;

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
    let mut game_tick = GameTick::default();

    if cfg!(debug_assertions) {
        // Debug only
        // Entity for debug lines
        world.spawn((DebugLines(Vec::new()),));
        world.spawn((DebugData::new(),));
    }

    world.spawn((
        MainCamera,
        CameraZoom(0.0025),
        Transform {
            position: vec2(0.0, 0.0),
            ..Default::default()
        },
    ));

    asset_server
        .load_assets(&["assets/player.png", "assets/enemy.png"])
        .await;

    spawn_player(&mut world);

    loop {
        clear_background(GRAY);

        player_input_system(&mut world, &mut physics_ressources);

        // Game tick related
        game_tick.accumulator += get_frame_time();
        while game_tick.accumulator >= game_tick.tick_rate {
            // Physic
            physics_cleanup_system(&mut world, &mut physics_ressources);
            sync_physics_world(&mut world, &mut physics_ressources);
            physics_step_system(&mut physics_ressources, &game_tick);
            collision_register(&mut world, &physics_ressources);

            // Game
            enemy_ai_system(&mut world, &mut physics_ressources);
            enemy_spawner_system(&mut world, &mut enemy_spawner);

            detect_player_dead(&mut world);
            detect_projectiles_collision(&mut world);
            detect_enemy_dead(&mut world);

            game_tick.accumulator -= game_tick.tick_rate;
            game_tick.ticks_elapsed += 1;
        }

        sync_transforms(&mut world, &physics_ressources, &game_tick);

        if cfg!(debug_assertions) {
            // Debug only
            // Dessine les boîtes de collision pour le débogage
            debug_draw_colliders_system(&mut world, &physics_ressources);
            debug_infos_system(&mut world, &game_tick);
        }

        draw_world(&mut world, &asset_server);
        // Send frame
        next_frame().await
    }
}
