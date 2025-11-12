use hecs::World;
use macroquad::prelude::*;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::{
    asset_server::AssetServer,
    components::{Enemy, Player, Speed, Sprite, Transform},
    physic::{PhysicsResources, RigidBodyHandleComponent},
};


pub struct EnemySpawner {
    pub timer: f32,
    pub spawn_interval: f32,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            timer: 0.0,
            spawn_interval: 2.0,
        }
    }
}

pub fn enemy_spawner_system(
    world: &mut World,
    asset_server: &AssetServer,
    spawner: &mut EnemySpawner,
) {
    spawner.timer += get_frame_time();

    if spawner.timer >= spawner.spawn_interval {
        spawner.timer = 0.0;

        let enemy_texture = asset_server.get_texture("assets/ennemy.png").unwrap().clone();

        let enemy_body = RigidBodyBuilder::dynamic().lock_rotations().build();
        let enemy_collider = ColliderBuilder::cuboid(16., 16.)
            .translation([32. / 2., 32. / 2.].into())
            .build();

        // Apparaît à une position fixe pour l'exemple.
        let spawn_position = vec2(200.0, 200.0);

        world.spawn((
            Enemy,
            Transform(spawn_position),
            Speed(100.0),
            Sprite {
                texture: enemy_texture,
                scale: 1.0,
            },
            enemy_body,
            enemy_collider,
        ));
    }
}

pub fn enemy_ai_system(world: &mut World, physics: &mut PhysicsResources) {
    let mut player_pos = None;
    for (_id, (transform,)) in world.query::<(&Transform,)>().with::<&Player>().iter() {
        player_pos = Some(transform.0);
        break; 
    }

    if let Some(player_pos) = player_pos {
        for (_id, (transform, speed, rb_handle)) in world.query_mut::<(&Transform, &Speed, &RigidBodyHandleComponent)>().with::<&Enemy>() {
            let direction = (player_pos - transform.0).normalize_or_zero();
            let desired_velocity = direction * speed.0;
            if let Some(body) = physics.rigid_body_set.get_mut(rb_handle.0) {
                body.set_linvel([desired_velocity.x, desired_velocity.y].into(), true);
            }
        }
    }
}