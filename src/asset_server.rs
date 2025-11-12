use std::collections::HashMap;
use macroquad::prelude::*;

#[derive(Debug)]
pub enum AssetState<T> {
    Loading,
    Loaded(T),
}

#[derive(Debug)]
pub struct AssetServer {
    textures: HashMap<String, AssetState<Texture2D>>,
}

impl AssetServer {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    // Pre-load textures before usage
    pub fn load_texture(&mut self, path: &str) {
        let path_str = path.to_string();
        if self.textures.contains_key(&path_str) {
            return;
        }

        self.textures.insert(path_str.clone(), AssetState::Loading);

        let future = async move {
            load_texture(&path_str).await
        };
    }

    pub fn get_texture(&self, path: &str) -> Option<&Texture2D> {
        match self.textures.get(path) {
            Some(AssetState::Loaded(texture)) => Some(texture),
            _ => None,
        }
    }

    async fn load_and_store_texture(&mut self, path: &str) {
        let texture = load_texture(path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        self.textures.insert(path.to_string(), AssetState::Loaded(texture));
    }

    // Load all initial assets
    pub async fn prime_assets(&mut self) {
        self.load_and_store_texture("assets/player.png").await;
        self.load_and_store_texture("assets/ennemy.png").await;
        
    }
}