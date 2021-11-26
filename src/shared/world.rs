use serde::{Deserialize, Serialize};

use super::{object::Object, player::Player, terrain::TerrainState};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct World {
    pub players: Vec<Player>,
    pub offline_players: Vec<Player>,
    pub terrain_chunks: Vec<((usize, usize), Vec<TerrainState>)>,
    pub objects: Vec<Object>
}

impl World {
    pub fn new() -> World {
        World {
            players: vec![],
            offline_players: vec![],
            terrain_chunks: vec![((0, 0), vec![TerrainState::Test; 1024])],
            objects: vec![]
        }
    }
    pub fn empty() -> World {
        World {
            players: vec![],
            offline_players: vec![],
            terrain_chunks: vec![],
            objects: vec![]
        }
    }
}
