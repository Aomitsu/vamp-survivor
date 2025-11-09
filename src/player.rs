use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::physic::Physics;

pub struct Player {
    texture: Texture2D,
    position: Vec2,
    rotation: f32,
    speed: f32,
    rigid_body_handle: RigidBodyHandle,
    _collider_handle: ColliderHandle,
}

impl Player {
    pub async fn new(phys: &mut Physics) -> Self {
        let texture = load_texture("assets/player.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![200.0, 200.0])
            .linear_damping(5.0)
            .build();
        let collider = ColliderBuilder::cuboid(32., 32.)
            .build();

        let phys_handle = phys.register_with_parent(rigid_body, collider);

        Player {
            texture: texture,
            position: vec2(200.0, 200.0),
            rotation: 0.,
            speed: 5.0 * 10000.0,
            rigid_body_handle: phys_handle.0,
            _collider_handle: phys_handle.1
        }
    }

    pub fn update(&mut self, phys: &mut Physics) {
        // Get physics info
        if let Some(rigid_body) = phys.get_rigid_body_set().get(self.rigid_body_handle) {
            let pos = rigid_body.position();
            self.position = vec2(pos.translation.x, pos.translation.y);
        }

        let mut move_vec = Vec2::ZERO;
        let keys_down = get_keys_down();
        for key in keys_down {
            match key {
                KeyCode::W | KeyCode::Up => move_vec.y -= 1.0,
                KeyCode::S | KeyCode::Down => move_vec.y += 1.0,
                KeyCode::A | KeyCode::Left => move_vec.x -= 1.0,
                KeyCode::D | KeyCode::Right => move_vec.x += 1.0,
                _ => (),
            }
        }

        if let Some(rigid_body) = phys.get_rigid_body_set_mut().get_mut(self.rigid_body_handle) {
            // Movement
            // TODO: MOVE WITH FORCES
            move_vec = move_vec.normalize_or_zero();
            rigid_body.apply_impulse(vector![move_vec.x * self.speed, move_vec.y * self.speed], true);
        }
        // Rotate character depending on mouse location, don't move collider box (useless)
        let mouse_pos = mouse_position_local();
        self.rotation = mouse_pos.y.atan2(mouse_pos.x);

    }

    pub fn draw(&self) {
        let params = DrawTextureParams {
            rotation: self.rotation,
            ..Default::default()
        };
        draw_texture_ex(&self.texture, self.position.x, self.position.y, WHITE, params);
    }

    pub fn position_centred(&self) -> Vec2 {
        self.position + vec2(self.texture.width() / 2.0, self.texture.height() / 2.0)
    }
}