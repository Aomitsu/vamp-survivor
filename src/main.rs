use macroquad::prelude::*;

use crate::debug::Debug;

mod map;
mod debug;

fn window_conf() -> Conf {
    Conf {
        window_title: "vamp-survivor".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut debug = Debug::new();
    loop {
        debug.update();

        
        clear_background(DARKGRAY);
        debug.draw();
        next_frame().await
    }
}