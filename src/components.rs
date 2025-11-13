use macroquad::prelude::*;

pub struct Transform(pub Vec2);
pub struct Speed(pub f32);
pub struct Sprite {
    pub texture: Texture2D,
    pub scale: f32
}
pub struct Health{
    pub actual: f32,
    pub max: f32
}
pub struct Damage(pub f32);
pub struct Player;
pub struct Enemy;
