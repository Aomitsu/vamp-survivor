use hecs::World;
use macroquad::prelude::*;
use rapier2d::prelude::*;
use vamp_survivor::despawn_phys_entity;

use crate::{
    asset_server::{self},
    components::{
        gameplay::{Damage, Health, Speed}, physic::RigidBodyHandleComponent, render::Sprite, tags::{Enemy, Player}, transform::Transform
    },
    physic::PhysicsResources,
};

pub struct EnemySpawner {
    pub timer: f32,
    pub spawn_interval: f32,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            timer: 0.0,
            spawn_interval: 0.5,
        }
    }
}

pub fn enemy_spawner_system(world: &mut World, spawner: &mut EnemySpawner) {
    spawner.timer += get_frame_time();

    if spawner.timer >= spawner.spawn_interval {
        spawner.timer = 0.0;

        // Apparaît à une position fixe pour l'exemple.
        let spawn_position = vec2(200.0, 200.0);

        let enemy_body = RigidBodyBuilder::dynamic()
            .translation([spawn_position.x, spawn_position.y].into())
            .lock_rotations()
            .build();
        let enemy_collider = ColliderBuilder::cuboid(16., 16.)
            .translation([32. / 2., 32. / 2.].into())
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();

        world.spawn((
            Enemy,
            Transform {
                position: spawn_position,
                ..Default::default()
            },
            Speed(80.0),
            Damage(10.0),
            Health{
                actual: 100.0,
                max: 100.0
            },
            Sprite {
                asset_id: asset_server::assets::enemy(),
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
        player_pos = Some(transform.position);
    }

    if let Some(player_pos) = player_pos {
        for (_id, (transform, speed, rb_handle)) in world
            .query_mut::<(&Transform, &Speed, &RigidBodyHandleComponent)>()
            .with::<&Enemy>()
        {
            let direction = (player_pos - transform.position).normalize_or_zero();
            let desired_velocity = direction * speed.0;
            if let Some(body) = physics.rigid_body_set.get_mut(rb_handle.0) {
                body.set_linvel([desired_velocity.x, desired_velocity.y].into(), true);
            }
        }
    }
}

pub fn detect_enemy_dead(world: &mut World) {
    let mut to_despawn = Vec::new();
    for (id, health) in world.query::<&Health>().with::<&Enemy>().iter() {
        if health.actual <= 0.0 {
            to_despawn.push(id);
        }
    }
    for id in to_despawn {
        despawn_phys_entity!(world, id);
    }
}
