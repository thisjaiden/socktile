use crate::{components::GamePosition, shared::terrain::TerrainState};

pub struct Reality {
    player_position: GamePosition
}

impl Reality {
    pub fn init() -> Reality {
        Reality {
            player_position: GamePosition { x: -25565.0, y: -25565.0 }
        }
    }
    pub fn set_player_position(&mut self, position: GamePosition) {
        self.player_position = position;
    }
    pub fn add_chunk(&mut self, chunk_position: (usize, usize), chunk_data: Vec<TerrainState>) {
        
    }
}