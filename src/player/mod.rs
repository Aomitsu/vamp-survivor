pub mod abilities;

use std::ops::{Deref, DerefMut};

use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{entity::Entity, physic::Physics, player::abilities::{Ability, FireballAbility}};


pub struct Player {
    base: Entity,
    abilities: Vec<Box<dyn Ability>>
}

impl Deref for Player {
    type Target = Entity;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
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

        let mut base = Entity::new_with_phys(texture, position, phys_handle.0, phys_handle.1);
        base.speed = 10 * 1000;
        let abilities: Vec<Box<dyn Ability>> = Vec::new();


        // Create
        let mut player = Player {
            base,
            abilities
        };

        // Give Fireball ability
        player.give_ability(Box::new(FireballAbility::new().await));

        player
    }

    pub fn update(&mut self, phys: &mut Physics) {
        // Get physics data to render character
        if let Some(collider) = phys.get_collider_set().get(self.collider_handle.unwrap()) {
            let pos = collider.position();
            let position = vec2(pos.translation.x, pos.translation.y);
            let centred = position + vec2(self.texture.width() / -2.0, self.texture.height() / -2.0);
            self.position = centred;
        }

        // Move character with user inputs 
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

        if let Some(rigid_body) = phys.get_rigid_body_set_mut().get_mut(self.rigid_body_handle.unwrap()) {
            // Movement
            move_vec = move_vec.normalize_or_zero();
            rigid_body.apply_impulse(vector![move_vec.x * self.speed as f32, move_vec.y * self.speed as f32], true);
        }
        // Rotate character depending on mouse location, don't move collider box (useless)
        let mouse_pos = mouse_position_local();
        self.rotation = mouse_pos.y.atan2(mouse_pos.x);

        self.abilities.iter().for_each(|boxxed_ability| {
            boxxed_ability.update(phys, &self.base);
        });

    }

    pub fn draw(&mut self) {
        let params = DrawTextureParams {
            rotation: self.rotation,
            ..Default::default()
        };
        draw_texture_ex(&self.texture, self.position.x, self.position.y, WHITE, params);

        // Draw abilities

        self.abilities.iter().for_each(|boxxed_ability| {
            boxxed_ability.draw();
        });
    }

    // TODO: Move to entity
    pub fn position_centred(&self) -> Vec2 {
        self.position + vec2(self.texture.width() / 2.0, self.texture.height() / 2.0)
    }

    pub fn give_ability(&mut self, ability: Box<dyn Ability>) {
        // Get the TypeId of the ability we're trying to give.
        let new_ability_type_id = ability.as_any().type_id();

        // Find if an ability of the same type already exists.
        if let Some(existing_ability) = self.abilities.iter_mut().find(|a| a.as_any().type_id() == new_ability_type_id) {
            // If it exists, level it up.
            let data = existing_ability.get_data().clone();

            // TODO: Upgrade it

            existing_ability.set_data(&data);
        } else {
            // If it doesn't exist, add it to the list.
            self.abilities.push(ability);
        }
    }



}