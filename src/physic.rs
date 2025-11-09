use rapier2d::prelude::*;
use macroquad::prelude::*;

/// Physic implementation
/// using rapier.
/// 
/// Used for Player, Ennemies, bullets, etc...
pub struct Physics{
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: ()
}

pub struct PhysicsDebugInfo {
    pub rbody_set_size: usize,
    pub collider_set_size: usize,
}


impl Physics {
    /// Create new physic universe
    pub fn new() -> Self {
        let rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        collider_set.insert(collider);

        Physics { rigid_body_set, collider_set, integration_parameters, physics_pipeline, island_manager, broad_phase, narrow_phase, impulse_joint_set, multibody_joint_set, ccd_solver, physics_hooks, event_handler }

    }

    /// Update the physic universe
    pub fn update(&mut self) {
        // Set gravity to none, top down view so useless.
        let gravity = vector![0.0, 0.0];
        // Using frametime
        self.integration_parameters.dt = get_frame_time();

        // Update physics
        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler
        );
    }

    // Draw debug lines such as colliders.
    pub fn draw(&self, is_debug: bool) {
        if !is_debug {return;}
        // For each colliders
        for (_handle, collider) in self.collider_set.iter() {
            let position = collider.position();
            let shape = collider.shape();

            // Draw the collider
            if let Some(cuboid) = shape.as_cuboid() {
                let extents = cuboid.half_extents;
                let width = extents.x * 2.0;
                let height = extents.y * 2.0;

                // Collider pos is at center in rapier, adapt it to macroquad position.
                let top_left_x = position.translation.x - extents.x;
                let top_left_y = position.translation.y - extents.y;

                // Draw collider
                draw_rectangle_lines(top_left_x, top_left_y, width, height, 2.0, LIME);
            }
        }
    }

    pub fn debug_info(&self) -> PhysicsDebugInfo {
        // return some debug infos
        PhysicsDebugInfo { rbody_set_size: self.rigid_body_set.len(), collider_set_size: self.collider_set.len() }
    }

    /// Register rigibody with collider, and set parental relation.
    pub fn register_with_parent(&mut self, rigid_body: RigidBody, collider: Collider) -> (RigidBodyHandle, ColliderHandle) {
        let handle_rigid_body: RigidBodyHandle = self.rigid_body_set.insert(rigid_body);
        let handle_collider: ColliderHandle = self.collider_set.insert_with_parent(collider, handle_rigid_body, &mut self.rigid_body_set);
        (handle_rigid_body, handle_collider)
    
    }

    pub fn _get_rigid_body_set(&self) -> &RigidBodySet {
        &self.rigid_body_set
    }

    pub fn get_collider_set(&self) -> &ColliderSet {
        &self.collider_set
    }

    pub fn get_rigid_body_set_mut(&mut self) -> &mut RigidBodySet {
        &mut self.rigid_body_set
    }
}
