use std::{any::Any, ops::{Deref, DerefMut}};

use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{entity::Entity, physic::Physics, player::Player};

#[derive(Clone, Default)]
pub struct AbilityData {
    level: i64,
    damage: i64,
    cooldown: f64,
}

pub trait Ability: Any {
    fn update(&self, phys: &mut Physics, player: &Entity);
    fn draw(&self);

    fn get_data(&self) -> &AbilityData;
    fn set_data(&mut self, data: &AbilityData);

    fn as_any(&self) -> &dyn Any;
}

pub struct FireballAbility{
    data: AbilityData,
    tileset_index: i64,
    base_texture: Texture2D,
    entities: Vec<Entity>
}

impl Ability for FireballAbility {
    fn update(&self, phys: &mut Physics, player: &Entity) {
        todo!()
    }

    fn draw(&self) {
        todo!()
    }

    fn get_data(&self) -> &AbilityData {
        &self.data
    }

    fn set_data(&mut self, data: &AbilityData) {
        self.data = data.clone()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

}

impl FireballAbility {
    pub async fn new() -> Self {
        let base_texture = load_texture("assets/projectiles/fireball.png").await.unwrap();
        base_texture.set_filter(FilterMode::Nearest);
        Self {
            data: AbilityData {
                level: 1,
                damage: 10,
                cooldown: 300.,
            },
            base_texture, 
            tileset_index: 0,
            entities: Vec::new()
        }
    }
}