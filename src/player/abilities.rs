use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::physic::Physics;

pub struct PlayerAbilities {
    global_cast_speed: f32, // in ms
    last_cast_time: f64,
    fireball: Option<PAFireball>,
}

impl PlayerAbilities {

    pub async fn new() -> Self {
        PlayerAbilities { 
            global_cast_speed: 1000.0,
            last_cast_time: 0.0,
            fireball: PAFireball::new().await.into(),
        }
    }

    pub fn update(&mut self, mut phys: &mut Physics, player_pos: Vec2) {
        // each global_cast_speed ms
        let time = get_time(); // Time in seconds
        if time - self.last_cast_time >= (self.global_cast_speed / 1000.0) as f64 {
            
            // TODO: Independant cast speed
            self.fireball.as_mut().unwrap().cast(&mut phys, player_pos);



            self.last_cast_time = time;
        }

        if let Some(fireball) = &mut self.fireball {
            fireball.update(phys);
        }

    }

    pub fn draw(&mut self) {
        if let Some(fireball) = &mut self.fireball {
            fireball.draw();
        }
    }

}

impl Default for PlayerAbilities {
    fn default() -> Self {
        Self { global_cast_speed: 1000.0, fireball: Default::default(), last_cast_time: 0.0 }
    }
}

struct PAFireball {
    // Macroquad
    texture: Texture2D,
    texture_index: i8,
    tile_size: i32,
    animation_speed: f32, // in ms
    last_frame_update: f64,
    // Physic
    speed: i32,
    // Info
    fireball_database: Vec<FireballData>,
}

struct FireballData {
    // Render
    position: Vec2,
    rotation: f32,
    // Physic
    rigid_body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    // Data
    health: i32,
}

impl PAFireball {
    pub async fn new() -> Self {
        let texture = load_texture("assets/proj/fireball.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);


        PAFireball {
            texture,
            speed: 100 * 1000,
            fireball_database: Vec::new(),
            texture_index: 0,
            tile_size: 16,
            animation_speed: 200.0,
            last_frame_update: 0.0,
        }
    }

    pub fn cast(&mut self, phys: &mut Physics, position: Vec2) {
        let collider = ColliderBuilder::cuboid(5., 5.)
            .translation(vector![10., 10.])
            .sensor(true)
            .build();
        let mut rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![position.x, position.y])
            .build();

        // Create a static force going in the mouse location direction
        let mouse_pos = mouse_position_local();
        let y = mouse_pos.y.atan2(mouse_pos.x);
        let x = mouse_pos.x.atan2(mouse_pos.y);

        rigid_body.add_force(vector![x * self.speed as f32, y * self.speed as f32], true);


        let phys_handle = phys.register_with_parent(rigid_body, collider);
        
        self.fireball_database.push(
            FireballData { 
                position: vec2(0.0, 0.0),
                rotation: 0.,
                rigid_body_handle: phys_handle.0,
                collider_handle: phys_handle.1,
                health: 100
            }
        );
    }

    pub fn update(&mut self, phys: &mut Physics) {
        let mut i = 0;
        while i < self.fireball_database.len() {
            let fireball = &mut self.fireball_database[i];
            if let Some(collider) = phys.get_collider_set().get(fireball.collider_handle) {
                fireball.position = vec2(collider.position().translation.x - 7., collider.position().translation.y - 7.)
            } else {
                // Remove if no physic body related
                self.fireball_database.remove(i);
            }
            i += 1;
        }

        // Animate texture
        let time = get_time();
        if time - self.last_frame_update >= (self.animation_speed / 1000.0) as f64 {
            self.texture_index += 1;
            if self.texture_index >= 3 {
                self.texture_index = 0;
            }
            self.last_frame_update = time;
        }
    }

    pub fn draw(&mut self) {
        for fireball in &mut self.fireball_database {
            let params = DrawTextureParams {
                source: Some(Rect::new(
                    self.texture_index as f32 * self.tile_size as f32,
                    0.0,
                    self.tile_size as f32,
                    self.tile_size as f32,
                )),
                rotation: fireball.rotation,
                ..Default::default()
            };
            draw_texture_ex(
                &self.texture,
                fireball.position.x,
                fireball.position.y, 
                WHITE,
                params
            );
        }
    }
}
