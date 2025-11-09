use macroquad::prelude::*;

use crate::{debug::Debug, physic::Physics, player::Player};

mod map;
mod debug;
mod player;
mod physic;

fn window_conf() -> Conf {
    Conf {
        window_title: "vamp-survivor".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut debug = Debug::new();
    let mut physic = Physics::new();
    let mut player = Player::new(&mut physic).await;
    loop {
        debug.update();
        physic.update();
        player.update(&mut physic);
        
        let zoom_level = 0.005;
        set_camera(&Camera2D {
            zoom: vec2(zoom_level, zoom_level * (screen_width() / screen_height())),
            target: player.position_centred(),
            ..Default::default()
        });

        // Drawing in space
        clear_background(DARKGRAY);
        player.draw();
 
        // Drawing on screen
        set_default_camera();
        debug.draw();
        physic.draw();

        // Send frame
        next_frame().await
    }
}