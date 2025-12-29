/// Note : Some components are located in a specific file for readability :
/// Physics components are in `physic.rs`

use macroquad::prelude::*;

use crate::asset_server::AssetId;

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

pub struct Speed(pub f32);
pub struct Health {
    pub actual: f32,
    #[allow(dead_code)]
    pub max: f32,
}
#[allow(dead_code)]
pub struct Damage(pub f32);

// Drawing
pub struct Sprite {
    pub asset_id: AssetId,
    pub scale: f32,
}
/// Text component to draw text.
/// TODO: Make it more customizable, with [macroquad TextParams](https://docs.rs/macroquad/latest/macroquad/text/struct.TextParams.html)
pub struct Text {
    pub text: String,
    pub color: Color,
}

// Specific
pub struct Player;
pub struct Enemy;
/// Marker component for entities that should be despawned at the end of the frame.
pub struct Despawn;

/// Resource to handle Fixed Update Logic (Tick)
pub struct GameTick {
    pub tick_rate: f32,
    pub ticks_elapsed: u32,
    pub accumulator: f32,
}

impl Default for GameTick {
    fn default() -> Self {
        Self {
            tick_rate: 1.0 / 32.0,
            ticks_elapsed: 0,
            accumulator: 0.0,
        }
    }
}
