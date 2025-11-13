use hecs::World;
use macroquad::prelude::*;

use crate::physic::PhysicsResources;

/// Un composant qui contiendra une liste de lignes à dessiner pour le débogage.
/// On peut imaginer une seule entité "Debug" dans le monde qui possède ce composant.
pub struct DebugLines(pub Vec<LineInfo>);

pub struct LineInfo {
    pub from: Vec2,
    pub to: Vec2,
    pub thickness: f32,
    pub color: Color,
}

impl DebugLines {
    /// Ajoute une ligne à dessiner pour la frame actuelle.
    pub fn draw_line(&mut self, from: Vec2, to: Vec2, thickness: f32, color: Color) {
        self.0.push(LineInfo { from, to, thickness, color });
    }
}

/// Draw rapier collide box
pub fn debug_draw_colliders_system(world: &mut World, physics: &PhysicsResources) {
    let debug_lines = if let Some((_id, lines)) = world.query_mut::<&mut DebugLines>().into_iter().next() {
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

