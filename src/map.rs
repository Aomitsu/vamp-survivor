use macroquad::prelude::*;
use noise::{Fbm, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};

pub struct Map{
    tileset: Texture2D,
    width: i32,
    height: i32,
    tile_size: f32,
    tiles: Vec<Vec<i32>>,
}

impl Map {
    pub async fn new() -> Self {
        let tileset = load_texture("assets/floor_tileset.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);

        let width = 500;
        let height = 500;

        Map {
            tileset,
            width: width,
            height: height,
            tile_size: 32.0,
            tiles: vec![vec![0; width as usize]; height as usize],
        }
    }

    pub fn update(&self) {

    }

    pub fn draw(&self, player_pos: &Vec2) {
        let zoom_level = 0.003;
        let aspect_ratio = screen_width() / screen_height();


        let view_width = 2.0 / zoom_level;
        let view_height = 2.0 / (zoom_level * aspect_ratio);


        let start_x = ((player_pos.x - view_width / 2.0) / self.tile_size).floor() as i32 - 1;
        let end_x = ((player_pos.x + view_width / 2.0) / self.tile_size).ceil() as i32 + 1;
        let start_y = ((player_pos.y - view_height / 2.0) / self.tile_size).floor() as i32 - 1;
        let end_y = ((player_pos.y + view_height / 2.0) / self.tile_size).ceil() as i32 + 1;


        let start_x = start_x.max(0);
        let end_x = end_x.min(self.width);
        let start_y = start_y.max(0);
        let end_y = end_y.min(self.height);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let tile_id = self.tiles[y as usize][x as usize];
                let dest_x = x as f32 * self.tile_size;
                let dest_y = y as f32 * self.tile_size;

                let source_rect = Rect::new(tile_id as f32 * self.tile_size, 0., self.tile_size, self.tile_size);
                draw_texture_ex(
                    &self.tileset,
                    dest_x, dest_y,
                    WHITE,
                    DrawTextureParams { source: Some(source_rect), ..Default::default() },
                );
            }
        }
    }

    pub fn generate_chunk(&mut self) {
        let width: usize = self.width as usize;
        let height: usize  = self.height as usize;

        let fbm = Fbm::<Perlin>::new(25565);
        let noise_map = PlaneMapBuilder::new(&fbm).set_size(width, height).build();

        let mut tiles = vec![vec![0; width]; height];
        for y in 0..height {
            for x in 0..width {
                let noise_val = noise_map.get_value(x, y);

                // On associe la valeur de bruit (entre -1.0 et 1.0) à un ID de tuile (0 à 3)
                tiles[y][x] = if noise_val < -0.25 {
                    3
                } else if noise_val < 0.0 {
                    2
                } else if noise_val < 0.25 {
                    1
                } else {
                    0
                };
            }
        }
        self.tiles = tiles;
    }
}