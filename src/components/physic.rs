use hecs::Entity;
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

/// A component to hold the handle to the Rapier rigid body.
pub struct RigidBodyHandleComponent(pub RigidBodyHandle);

/// A component to hold the handle to the Rapier collider.
#[allow(dead_code)]
pub struct ColliderHandleComponent(pub ColliderHandle);

/// A component who list every entities who collide with
pub struct CollideWith(pub Vec<Entity>);