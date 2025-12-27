use std::sync::mpsc::{Receiver, Sender};

use hecs::{Entity, World};
use log::debug;
use macroquad::prelude::{get_frame_time, vec2};
use rapier2d::prelude::*;

use crate::components::Transform;

// A component to hold the handle to the Rapier rigid body.
pub struct RigidBodyHandleComponent(pub RigidBodyHandle);

// A component to hold the handle to the Rapier collider.
pub struct ColliderHandleComponent(pub ColliderHandle);

// A component who list every entities who collide with
pub struct CollideWith(pub Vec<Entity>);

/// A struct to hold all the Rapier physics resources.
/// This can be stored in your main game state and passed to systems.
pub struct PhysicsResources {
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub collision_event_sender: Sender<CollisionEvent>,
    pub collision_event_receiver: Receiver<CollisionEvent>,
    pub contact_force_sender: Sender<ContactForceEvent>,
    pub contact_force_receiver: Receiver<ContactForceEvent>,
}

/// Creates the initial physics resources.
pub fn setup_physics() -> PhysicsResources {
    let (collision_sender, collision_receiver) = std::sync::mpsc::channel();
    let (force_sender, force_receiver) = std::sync::mpsc::channel();

    PhysicsResources {
        integration_parameters: IntegrationParameters::default(),
        physics_pipeline: PhysicsPipeline::new(),
        island_manager: IslandManager::new(),
        broad_phase: DefaultBroadPhase::new(),
        narrow_phase: NarrowPhase::new(),
        rigid_body_set: RigidBodySet::new(),
        collider_set: ColliderSet::new(),
        impulse_joint_set: ImpulseJointSet::new(),
        multibody_joint_set: MultibodyJointSet::new(),
        ccd_solver: CCDSolver::new(),
        collision_event_sender: collision_sender,
        collision_event_receiver: collision_receiver,
        contact_force_sender: force_sender,
        contact_force_receiver: force_receiver,
    }
}

/// The main physics simulation system.
pub fn physics_step_system(physics: &mut PhysicsResources) {
    let gravity = vector![0.0, 0.0]; // Top-down, no gravity.
    physics.integration_parameters.dt = get_frame_time();

    let physics_hooks = ();
    let event_handler = ChannelEventCollector::new(
        physics.collision_event_sender.clone(),
        physics.contact_force_sender.clone(),
    );

    physics.physics_pipeline.step(
        &gravity,
        &physics.integration_parameters,
        &mut physics.island_manager,
        &mut physics.broad_phase,
        &mut physics.narrow_phase,
        &mut physics.rigid_body_set,
        &mut physics.collider_set,
        &mut physics.impulse_joint_set,
        &mut physics.multibody_joint_set,
        &mut physics.ccd_solver,
        &physics_hooks,
        &event_handler,
    );
}

/// A system that finds entities with `RigidBody` and `Collider` components
/// and adds them to the physics world.
pub fn sync_physics_world(world: &mut World, physics: &mut PhysicsResources) {
    let mut commands = Vec::<(
        Entity,
        RigidBodyHandleComponent,
        ColliderHandleComponent,
        CollideWith,
    )>::new();
    // Query for entities that have a body and collider but no handle yet.
    for (entity, (body, collider)) in world
        .query_mut::<(&RigidBody, &mut Collider)>()
        .without::<&RigidBodyHandleComponent>()
    {
        // Store the entity's bits in the collider's user_data field.
        collider.user_data = entity.to_bits().get() as u128;
        let body_handle = physics.rigid_body_set.insert(body.clone());
        let collider_handle = physics.collider_set.insert_with_parent(
            collider.clone(),
            body_handle,
            &mut physics.rigid_body_set,
        ); // This line was missing from the original context
        commands.push((
            entity,
            RigidBodyHandleComponent(body_handle),
            ColliderHandleComponent(collider_handle),
            CollideWith(Vec::new()),
        ));
    }

    // Add the handles as components to the entities.
    for (entity, body_handle, collider_handle, collide_with) in commands {
        world
            .insert(entity, (body_handle, collider_handle, collide_with))
            .unwrap();
        debug!("Insert Physics components to new entity {:?}", entity)
    }
}

/// A system to update the `Transform` of entities based on the physics simulation.
pub fn sync_transforms(world: &mut World, physics: &PhysicsResources) {
    for (_entity, (transform, body_handle)) in
        world.query_mut::<(&mut Transform, &RigidBodyHandleComponent)>()
    {
        if let Some(body) = physics.rigid_body_set.get(body_handle.0) {
            transform.0 = vec2(body.translation().x, body.translation().y);
        }
    }
}

/// Helper function to extract entities from a collision event.
pub fn get_entities_from_collision(
    event: CollisionEvent,
    colliders: &ColliderSet,
) -> Option<(Entity, Entity, CollisionEvent)> {
    let (handle1, handle2) = match event {
        CollisionEvent::Started(h1, h2, _) => (h1, h2),
        CollisionEvent::Stopped(h1, h2, _) => (h1, h2),
    };

    let collider1 = colliders.get(handle1)?;
    let collider2 = colliders.get(handle2)?;

    let entity1 = Entity::from_bits(collider1.user_data as u64)?;
    let entity2 = Entity::from_bits(collider2.user_data as u64)?;

    Some((entity1, entity2, event))
}
pub fn collision_register(world: &mut World, physics: &PhysicsResources) {
    while let Ok(collision_event) = physics.collision_event_receiver.try_recv() {
        if let Some((entity1, entity2, event)) =
            get_entities_from_collision(collision_event, &physics.collider_set)
        {
            match event {
                CollisionEvent::Started(_, _, _) => {
                    debug!("Collision started between {:?} and {:?}", entity1, entity2);
                    if let Ok(collide_with) = world.query_one_mut::<&mut CollideWith>(entity1) {
                        collide_with.0.push(entity2);
                    }
                    if let Ok(collide_with) = world.query_one_mut::<&mut CollideWith>(entity2) {
                        collide_with.0.push(entity1);
                    }
                }
                CollisionEvent::Stopped(_, _, _) => {
                    debug!("Collision stopped between {:?} and {:?}", entity1, entity2);
                    if let Ok(collide_with) = world.query_one_mut::<&mut CollideWith>(entity1) {
                        if let Some(index) = collide_with
                            .0
                            .iter()
                            .position(|value| value.id() == entity2.id())
                        {
                            collide_with.0.remove(index);
                        }
                    }

                    if let Ok(collide_with) = world.query_one_mut::<&mut CollideWith>(entity2) {
                        if let Some(index) = collide_with
                            .0
                            .iter()
                            .position(|value| value.id() == entity1.id())
                        {
                            collide_with.0.remove(index);
                        }
                    }
                }
            }
        }
    }
}
