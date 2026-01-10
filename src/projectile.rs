use hecs::{Entity, World};
use macroquad::math::Vec2;
use rapier2d::prelude::{ActiveEvents, ColliderBuilder, RigidBodyBuilder};

use crate::{asset_server, components::{gameplay::{Damage, DamagePlayer, Health, Parent, Speed}, physic::CollideWith, render::Sprite, tags::{Enemy, Player, Projectile}, transform::Transform}};

pub enum ProjectileType {
    Fireball
}

pub fn spawn_projectile(world: &mut World, parent: Entity, _projectile_type: ProjectileType, origin: Vec2, direction: Vec2){

    let speed = 300.;

    // Query if entity is player or enemy
    let is_player = world.get::<&Player>(parent).is_ok();

    let mut projectile_body = RigidBodyBuilder::kinematic_velocity_based()
        .translation([origin.x, origin.y].into())
        .lock_rotations().build();
    let projectile_collider = ColliderBuilder::cuboid(8., 8.)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .translation([16. / 2., 16. / 2.].into())
        .sensor(true) // No physical existence
        .build();

    let desired_velocity = direction * speed;
    projectile_body.set_linvel([desired_velocity.x, desired_velocity.y].into(), true);

    world.spawn((
        Projectile,
        DamagePlayer(!is_player), // If parent is player, don't damage player.
        Speed(speed),
        Damage(1000.),
        Transform{
            position: origin,
            ..Default::default()
        },
        Sprite {
            asset_id: asset_server::assets::player(), // TODO: Temporary player for debug
            scale: 0.5,
        },
        Parent(parent),
        projectile_body,
        projectile_collider,
    ));
}

pub fn detect_projectiles_collision(world: &mut World) {
    for (_id, (_parent, _projectile, collide_with, damage_player, damage)) in world.query::<(&Parent, &Projectile, &CollideWith, &DamagePlayer, &Damage)>().iter() {

        for entity  in collide_with.0.iter() {
            let is_player = world.get::<&Player>(*entity).is_ok();
            if damage_player.0 && !is_player || !damage_player.0 && is_player {
                continue;
            }

            let health = world.get::<&mut Health>(*entity);
            if let Ok(mut health) = health {
                health.actual -= damage.0;
            } else {
                continue;
            }
        }
    }
}