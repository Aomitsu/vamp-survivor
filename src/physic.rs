use hecs::{World, Entity};
use macroquad::prelude::{get_frame_time, vec2};
use rapier2d::prelude::*;

use crate::components::Transform;

// A component to hold the handle to the Rapier rigid body.
pub struct RigidBodyHandleComponent(pub RigidBodyHandle);

// A component to hold the handle to the Rapier collider.
pub struct ColliderHandleComponent(pub ColliderHandle);

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
}

/// Creates the initial physics resources.
pub fn setup_physics() -> PhysicsResources {
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
    }
}

/// The main physics simulation system.
pub fn physics_step_system(physics: &mut PhysicsResources) {
    let gravity = vector![0.0, 0.0]; // Top-down, no gravity.
    physics.integration_parameters.dt = get_frame_time();

    let physics_hooks = ();
    let event_handler = ();

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
    let mut commands = Vec::<(Entity, RigidBodyHandleComponent, ColliderHandleComponent)>::new();
    // Query for entities that have a body and collider but no handle yet.
    for (entity, (body, collider)) in world.query_mut::<(&RigidBody, &Collider)>().without::<&RigidBodyHandleComponent>() {
        let body_handle = physics.rigid_body_set.insert(body.clone());
        let collider_handle = physics.collider_set.insert_with_parent(collider.clone(), body_handle, &mut physics.rigid_body_set);
        commands.push((entity, RigidBodyHandleComponent(body_handle), ColliderHandleComponent(collider_handle)));
    }

    // Add the handles as components to the entities.
    for (entity, body_handle, collider_handle) in commands {
        world.insert(entity, (body_handle, collider_handle)).unwrap();
    }
}

/// A system to update the `Transform` of entities based on the physics simulation.
pub fn sync_transforms(world: &mut World, physics: &PhysicsResources) {
    for (_entity, (transform, body_handle)) in world.query_mut::<(&mut Transform, &RigidBodyHandleComponent)>() {
        if let Some(body) = physics.rigid_body_set.get(body_handle.0) {
            transform.0 = vec2(body.translation().x, body.translation().y);
        }
    }
}