use bevy::prelude::*;

use crate::{components::GamePosition, shared::terrain::TerrainState};

pub struct Reality {
    player_position: GamePosition,
    camera: Camera
}

impl Reality {
    pub fn init() -> Reality {
        Reality {
            player_position: GamePosition { x: 0.0, y: 0.0 },
            camera: Camera::Static(GamePosition { x: 0.0, y: 0.0 })
        }
    }
    pub fn set_player_position(&mut self, position: GamePosition) {
        self.player_position = position;
    }
    pub fn add_chunk(&mut self, chunk_position: (usize, usize), chunk_data: Vec<TerrainState>) {
        todo!();
    }
    pub fn register_player(&mut self, player_eid: Entity) {
        todo!();
    }
    pub fn player_id(&mut self) -> Entity {
        todo!();
    }
}

pub enum Camera {
    PlayerPosition,
    Static(GamePosition),
    DriftToPlayer(GamePosition),
}