use futures::future::join_all;
use log::{error, info};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type AssetId = u64;

/// Why using hash as asset id and not string ?
/// cf. https://gameprogrammingpatterns.com/data-locality.html
///
///
/// 
/// 
pub struct AssetServer {
    textures: HashMap<AssetId, Texture2D>,
    missing_texture: Texture2D,
    // For debug, keep a link between ID and Path
    #[cfg(debug_assertions)]
    debug_names: HashMap<AssetId, String>,
}

impl AssetServer {
    pub fn new() -> Self {
        let missing_img = Image::gen_image_color(1, 1, MAGENTA);
        let missing_texture = Texture2D::from_image(&missing_img);

        Self {
            textures: HashMap::new(),
            missing_texture,
            #[cfg(debug_assertions)]
            debug_names: HashMap::new(),
        }
    }

    pub fn compute_id(path: &str) -> AssetId {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        hasher.finish()
    }

    pub fn get_texture(&self, id: AssetId) -> &Texture2D {
        match self.textures.get(&id) {
            Some(tex) => tex,
            None => {
                #[cfg(debug_assertions)]
                {
                    use log::error;

                    let name = self
                        .debug_names
                        .get(&id)
                        .map(|s| s.as_str())
                        .unwrap_or("UNKNOWN_ID");
                    error!("Texture ID {} ({}) not found!", id, name);
                }
                &self.missing_texture
            }
        }
    }

    pub async fn load_assets(&mut self, paths: &[&str]) {
        let mut futures = Vec::new();

        for path in paths {
            let path_owned = path.to_string();
            futures.push(async move {
                let result = load_texture(&path_owned).await;
                let id = AssetServer::compute_id(&path_owned);
                (id, path_owned, result)
            });
        }

        let results = join_all(futures).await;

        for (id, path, result) in results {
            match result {
                Ok(texture) => {
                    texture.set_filter(FilterMode::Nearest);
                    self.textures.insert(id, texture);

                    #[cfg(debug_assertions)]
                    self.debug_names.insert(id, path.clone());

                    info!("Asset Loaded: {} -> ID: {}", path, id);
                }
                Err(e) => error!("Loading asset {}: {}", path, e),
            }
        }
    }
}

pub mod assets {
    use super::AssetId;
    use super::AssetServer;

    pub fn player() -> AssetId {
        AssetServer::compute_id("assets/player.png")
    }
    pub fn enemy() -> AssetId {
        AssetServer::compute_id("assets/enemy.png")
    }
}
