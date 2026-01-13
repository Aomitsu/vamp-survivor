use macroquad::prelude::*;

pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            scale: vec2(1.0, 1.0),
            rotation: 0.0,
        }
    }
}
