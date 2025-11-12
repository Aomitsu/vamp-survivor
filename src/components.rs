use macroquad::prelude::*;

pub struct Position(pub Vec2);
pub struct Velocity(pub Vec2);
pub struct Speed(pub f32);
pub struct Sprite {
    pub texture: Texture2D,
    pub scale: f32
}
pub struct Player;
pub struct RectangleLines {
    pub size: Vec2,
    pub thicness: f32,
    pub color: Color
}
