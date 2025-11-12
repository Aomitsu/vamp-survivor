use std::default;

use hecs::World;
use macroquad::prelude::*;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::{asset_server::AssetServer, components::{Player, Speed, Sprite, Transform}, debug::{debug_draw_colliders_system, DebugLines}, enemy::{enemy_ai_system, enemy_spawner_system, EnemySpawner}, physic::{physics_step_system, setup_physics, sync_physics_world, sync_transforms}, player::player_input_system};

mod components;
mod player;
mod enemy;
mod physic;
#[cfg(debug_assertions)]
mod debug;
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
    let mut world = World::new();
    let mut physics_ressources = setup_physics();
    let mut asset_server = AssetServer::new();
    let mut enemy_spawner = EnemySpawner::default();

    if cfg!(debug_assertions) { // Debug only
        // Entity for debug lines
        world.spawn((DebugLines(Vec::new()),));
    }

    // Load initial assets
    asset_server.prime_assets().await;

    let player_body = RigidBodyBuilder::dynamic().lock_rotations().build();
    let player_collider = ColliderBuilder::cuboid(16., 16.)
        .translation([32./2., 32./2.].into())
        .build();

    // Get texture from AssetServer
    let player_texture = asset_server.get_texture("assets/player.png").unwrap().clone();
    world.spawn((
        Player,
        Transform(vec2(0.0, 0.0)),
        Speed(200.),
        Sprite { texture: player_texture, scale: 1.0 },
        player_body,
        player_collider
    ));

    loop {
        clear_background(GRAY);
        // Update physics
        sync_physics_world(&mut world, &mut physics_ressources);

        // Do things with entities
        player_input_system(&mut world, &mut physics_ressources);
        enemy_spawner_system(&mut world, &asset_server, &mut enemy_spawner);
        enemy_ai_system(&mut world, &mut physics_ressources);


        // Physics tick related
        physics_step_system(&mut physics_ressources);
        sync_transforms(&mut world, &mut physics_ressources);

        if cfg!(debug_assertions) { // Debug only
            // Dessine les boîtes de collision pour le débogage
            debug_draw_colliders_system(&mut world, &physics_ressources);
        }

        draw_world(&mut world);
        // Send frame
        next_frame().await
    }
}

fn draw_world(world: &mut World) {
    let zoom_level = 0.003;
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

    // Itère sur notre composant de débogage, dessine les lignes, puis nettoie le vecteur pour la prochaine frame.
    for (_id, debug_lines) in world.query_mut::<&mut DebugLines>() {
        for line in debug_lines.0.iter() {
            draw_line(line.from.x, line.from.y, line.to.x, line.to.y, line.thickness, line.color);
        }
        // Efface les lignes pour que de nouvelles puissent être ajoutées à la prochaine frame
        debug_lines.0.clear();
    }
}