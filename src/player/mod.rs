pub mod abilities;

use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{physic::Physics, player::abilities::PlayerAbilities};

pub struct Player {
    // Macroquad
    texture: Texture2D,
    position: Vec2,
    rotation: f32,
    // Physic
    speed: f32,
    rigid_body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    // Abilities
    player_abilities: PlayerAbilities
}

impl Player {
    pub async fn new(phys: &mut Physics) -> Self {
        let texture = load_texture("assets/player.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        let texture_data = texture.get_texture_data();
        let position = vec2(200.0, 200.0);
        
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![position.x, position.y])
            .linear_damping(5.0)
            .build();
        let collider = ColliderBuilder::cuboid(16., 16.)
            .translation(vector![texture_data.width as f32 / 2., texture_data.height as f32 / 2.])
            .build();

        let phys_handle = phys.register_with_parent(rigid_body, collider);
        
        Player {
            texture: texture,
            position,
            rotation: 0.,
            speed: 5.0 * 5000.0,
            rigid_body_handle: phys_handle.0,
            collider_handle: phys_handle.1,
            player_abilities: PlayerAbilities::new().await
        }
    }

    pub fn update(&mut self, phys: &mut Physics) {
        // Get physics info
        if let Some(collider) = phys.get_collider_set().get(self.collider_handle) {
            let pos = collider.position();
            let position = vec2(pos.translation.x, pos.translation.y);
            let centred = position + vec2(self.texture.width() / -2.0, self.texture.height() / -2.0);
            self.position = centred;
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
            move_vec = move_vec.normalize_or_zero();
            rigid_body.apply_impulse(vector![move_vec.x * self.speed, move_vec.y * self.speed], true);
        }
        // Rotate character depending on mouse location, don't move collider box (useless)
        let mouse_pos = mouse_position_local();
        self.rotation = mouse_pos.y.atan2(mouse_pos.x);

        // Abilities

        self.player_abilities.update(phys, self.position_centred());

    }

    pub fn draw(&mut self) {
        let params = DrawTextureParams {
            rotation: self.rotation,
            ..Default::default()
        };
        draw_texture_ex(&self.texture, self.position.x, self.position.y, WHITE, params);
        self.player_abilities.draw();
    }

    pub fn position_centred(&self) -> Vec2 {
        self.position + vec2(self.texture.width() / 2.0, self.texture.height() / 2.0)
    }
}