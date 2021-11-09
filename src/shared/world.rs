use serde::{Deserialize, Serialize};

use crate::components::GamePosition;

use super::player::Player;

pub const SPAWN_POSITION: GamePosition = GamePosition { x: 0.0, y: 0.0 };

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct World {
    pub players: Vec<Player>,
    pub offline_players: Vec<Player>,
}

impl World {
    pub fn new() -> World {
        World {
            players: vec![],
            offline_players: vec![],
        }
    }
}
