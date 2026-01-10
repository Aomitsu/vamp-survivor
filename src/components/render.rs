use macroquad::prelude::*;

use crate::asset_server::AssetId;

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
 