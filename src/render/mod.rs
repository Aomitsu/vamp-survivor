use hecs::World;
use macroquad::prelude::*;

use crate::{
    asset_server::AssetServer,
    components::{
        render::{CameraZoom, MainCamera, Sprite, Text},
        transform::Transform,
    },
    debug::debug_draw,
};

pub fn draw_world(world: &mut World, asset_server: &AssetServer) {
    let mut zoom_level = 0.0025;
    if let Some((id, _)) = world.query::<&MainCamera>().iter().next() {
        let cam_zoom = world.get::<&CameraZoom>(id);
        if let Ok(cam_zoom) = cam_zoom {
            zoom_level = cam_zoom.0;
        }
    } else {
        log::error!("No main camera found. Can't render world.");
        // TODO: Panic ? Integrate with PanicGator ??
        return;
    }

    let aspect_ratio = screen_width() / screen_height();

    let camera = Camera2D {
        zoom: vec2(zoom_level, zoom_level * aspect_ratio),
        ..Default::default()
    };

    set_camera(&camera);

    // Viewport for AABB Culling

    let min_screen = vec2(0.0, 0.0);
    let max_screen = vec2(screen_width(), screen_height());

    let min_world = camera.screen_to_world(min_screen);
    let max_world = camera.screen_to_world(max_screen);

    // Camera rendering cube

    let view_rect = Rect::new(
        min_world.x.min(max_world.x),
        min_world.y.min(max_world.y),
        (max_world.x - min_world.x).abs(),
        (max_world.y - min_world.y).abs(),
    );

    for (_id, (transform, sprite)) in &mut world.query::<(&Transform, &Sprite)>() {
        let texture = asset_server.get_texture(sprite.asset_id);
        let w = texture.width() * sprite.scale * transform.scale.x;
        let h = texture.height() * sprite.scale * transform.scale.y;

        let sprite_rect = Rect::new(transform.position.x, transform.position.y, w, h);

        if !view_rect.overlaps(&sprite_rect) {
            continue;
        }

        draw_texture_ex(
            texture,
            transform.position.x,
            transform.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                rotation: transform.rotation,
                ..Default::default()
            },
        )
    }

    for (_id, (pos, text)) in &mut world.query::<(&Transform, &Text)>() {
        draw_text_ex(
            text.text.as_str(),
            pos.position.x,
            pos.position.y,
            TextParams {
                color: text.color,
                ..Default::default()
            },
        );
    }

    if cfg!(debug_assertions) {
        draw_rectangle_lines(view_rect.x, view_rect.y, view_rect.w, view_rect.h, 0.1, RED);
        debug_draw(world);
    }
}
