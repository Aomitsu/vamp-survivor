use macroquad::prelude::*;

use crate::asset_server::AssetId;

pub struct Transform(pub Vec2);
pub struct Speed(pub f32);
pub struct Health {
    pub actual: f32,
    pub max: f32,
}
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
