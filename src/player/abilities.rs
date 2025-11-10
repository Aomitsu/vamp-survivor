use std::{any::Any, ops::{Deref, DerefMut}};

use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{entity::Entity, physic::Physics, player::Player};

#[derive(Clone, Default)]
pub struct AbilityData {
    level: i32,
    damage: i32,
}

pub trait Ability: Any {
    fn update(&self, phys: &mut Physics, player: &Entity);
    fn draw(&self);

    fn get_data(&self) -> &AbilityData;
    fn set_data(&mut self, data: &AbilityData);

    fn as_any(&self) -> &dyn Any;
}

#[derive(Default)]
pub struct FireballAbility{
    data: AbilityData,
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
    pub fn new() -> Self {
        Self {
            data: AbilityData {
                level: 1,
                damage: 10,
            }
        }
    }
}