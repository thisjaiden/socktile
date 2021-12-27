use bevy::{prelude::*, utils::HashMap};

use crate::{components::GamePosition, shared::terrain::TerrainState};

pub struct Reality {
    player_position: GamePosition,
    camera: Camera,
    chunks_to_load: Vec<(isize, isize)>,
    buffered_chunks: HashMap<(isize, isize), Vec<(usize, usize, TerrainState)>>
}

impl Reality {
    pub fn init() -> Reality {
        Reality {
            player_position: GamePosition { x: 0.0, y: 0.0 },
            camera: Camera::Static(GamePosition { x: 0.0, y: 0.0 }),
            chunks_to_load: vec![],
            buffered_chunks: HashMap::default()
        }
    }
    pub fn set_player_position(&mut self, position: GamePosition) {
        self.player_position = position.clone();
        self.camera = Camera::Static(position);
        // load visible world
        const ENV_WIDTH: f64 = 1920.0;
        const ENV_HEIGHT: f64 = 1088.0;
        let tile_x = (self.player_position.x / ENV_WIDTH).round() as isize;
        let tile_y = (self.player_position.y / ENV_HEIGHT).round() as isize;
        self.chunks_to_load.push((tile_x, tile_y));
    }
    pub fn system_chunk_loader(&mut self) {
        if !self.chunks_to_load.is_empty() {
            for chunk in self.chunks_to_load.clone() {
                
            }
        }
    }
    pub fn add_chunk(&mut self, chunk_position: (isize, isize), chunk_data: Vec<(usize, usize, TerrainState)>) {
        todo!();
    }
}

pub enum Camera {
    PlayerPosition,
    Static(GamePosition),
    DriftToPlayer,
}