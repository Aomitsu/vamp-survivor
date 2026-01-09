use std::collections::VecDeque;

use hecs::World;
use macroquad::prelude::*;

use crate::{components::GameTick, physic::PhysicsResources};

/// Un composant qui contiendra une liste de lignes à dessiner pour le débogage.
/// On peut imaginer une seule entité "Debug" dans le monde qui possède ce composant.
pub struct DebugLines(pub Vec<LineInfo>);

#[derive(Debug)]
#[allow(dead_code)]
pub struct DebugData {
    display: bool,
    page: i8,
    frame_times: VecDeque<f32>,
    avg_fps: i32,
    avg_frame_time: f32,
    sample_size: usize,
    last_tps_time: f64,
    last_tick_count: u32,
    tps: u32,
}

impl DebugData {
    pub fn new() -> Self {
        let sample_size = 120; // Moyenne sur les 120 dernières trames (environ 2 secondes à 60 FPS)
        Self {
            display: false,
            page: 0,
            frame_times: VecDeque::with_capacity(sample_size),
            avg_fps: 0,
            avg_frame_time: 0.0,
            sample_size,
            last_tps_time: get_time(),
            last_tick_count: 0,
            tps: 0,
        }
    }
}

pub struct LineInfo {
    pub from: Vec2,
    pub to: Vec2,
    pub thickness: f32,
    pub color: Color,
}

impl DebugLines {
    /// Ajoute une ligne à dessiner pour la frame actuelle.
    pub fn draw_line(&mut self, from: Vec2, to: Vec2, thickness: f32, color: Color) {
        self.0.push(LineInfo {
            from,
            to,
            thickness,
            color,
        });
    }
}

/// Draw rapier collide box
pub fn debug_draw_colliders_system(world: &mut World, physics: &PhysicsResources) {
    let debug_lines =
        if let Some((_id, lines)) = world.query_mut::<&mut DebugLines>().into_iter().next() {
            lines
        } else {
            return;
        };

    for (_collider_handle, collider) in physics.collider_set.iter() {
        if let Some(cuboid) = collider.shape().as_cuboid() {
            let half_extents = cuboid.half_extents;
            let position = collider.position();

            let corners = [
                Vec2::new(-half_extents.x, -half_extents.y),
                Vec2::new(half_extents.x, -half_extents.y),
                Vec2::new(half_extents.x, half_extents.y),
                Vec2::new(-half_extents.x, half_extents.y),
            ]
            .map(|p| vec2(position.translation.x, position.translation.y) + p);

            for i in 0..4 {
                let start_point = corners[i];
                let end_point = corners[(i + 1) % 4]; // Le modulo permet de boucler sur le dernier coin.
                debug_lines.draw_line(start_point, end_point, 1., GREEN);
            }
        }
    }
}

pub fn debug_infos_system(world: &mut World, game_tick: &GameTick) {
    for (_id, debug_data) in world.query_mut::<&mut DebugData>() {
        let frame_time = get_frame_time();
        debug_data.frame_times.push_back(frame_time);

        if debug_data.frame_times.len() > debug_data.sample_size {
            debug_data.frame_times.pop_front();
        }

        let total_time: f32 = debug_data.frame_times.iter().sum();
        if !debug_data.frame_times.is_empty() {
            debug_data.avg_frame_time = total_time / debug_data.frame_times.len() as f32;
            debug_data.avg_fps = (1.0 / debug_data.avg_frame_time) as i32;
        }

        let time = get_time();
        if time - debug_data.last_tps_time >= 1.0 {
            debug_data.tps = game_tick.ticks_elapsed.wrapping_sub(debug_data.last_tick_count);
            debug_data.last_tick_count = game_tick.ticks_elapsed;
            debug_data.last_tps_time = time;
        }

        log::info!(
            "FPS : {}; avg FPS : {}; Frame Time: {}; avg Frame Time {}",
            get_fps(),
            debug_data.avg_fps,
            get_frame_time(),
            debug_data.avg_frame_time,
        );
        log::info!(
            "Tick: {}; Accumulator: {}; Ticks per second: {}",
            game_tick.ticks_elapsed,
            game_tick.accumulator,
            debug_data.tps
            
        )
    }
}

pub fn debug_draw(world: &mut World) {
    // Search debug component to loop & draw all lines.
    for (_id, debug_lines) in world.query_mut::<&mut DebugLines>() {
        for line in debug_lines.0.iter() {
            draw_line(
                line.from.x,
                line.from.y,
                line.to.x,
                line.to.y,
                line.thickness,
                line.color,
            );
        }
        // Remove lines at each frame
        debug_lines.0.clear();
    }
}
