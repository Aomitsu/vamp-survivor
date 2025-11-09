use macroquad::prelude::*;

use crate::{debug::Debug, map::Map, physic::Physics, player::Player};

mod map;
mod debug;
mod player;
mod physic;

fn window_conf() -> Conf {
    Conf {
        window_title: "vamp-survivor".to_owned(),
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut debug = Debug::new();
    let mut physic = Physics::new();
    let mut player = Player::new(&mut physic).await;
    let mut map = Map::new().await;
    
    map.generate_chunk();
    
    loop {
        debug.update();
        map.update();
        player.update(&mut physic);
        physic.update();
        
        let zoom_level = 0.003;
        set_camera(&Camera2D {
            zoom: vec2(zoom_level, zoom_level * (screen_width() / screen_height())),
            target: player.position_centred(),
            ..Default::default()
        });

        // Drawing in space
        clear_background(DARKGRAY);
        map.draw(&player.position_centred());
        player.draw();
        physic.draw(debug.is_in_debug());
 
        // Drawing on screen
        set_default_camera();
        debug.draw(physic.debug_info());

        // Send frame
        next_frame().await
    }
}