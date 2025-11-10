use macroquad::prelude::*;
use rapier2d::prelude::*;

// Shared between all entities
pub struct Entity {
    // Render
    pub texture: Texture2D,
    pub position: Vec2,
    pub rotation: f32,
    // Physic
    pub speed: i32,
    pub rigid_body_handle: Option<RigidBodyHandle>,
    pub collider_handle: Option<ColliderHandle>,
}

impl Entity {
    pub fn new(texture: Texture2D, position: Vec2) -> Self {
        Entity { texture, position, rotation: 0., speed: 0, rigid_body_handle: None, collider_handle: None }
    }
    pub fn new_with_phys(texture: Texture2D, position: Vec2, rb_handle: RigidBodyHandle, coll_handle: ColliderHandle) -> Self {
        Entity { texture, position, rotation: 0., speed: 0, rigid_body_handle: Some(rb_handle), collider_handle: Some(coll_handle) }
    }

}