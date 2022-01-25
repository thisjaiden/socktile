use serde::{Deserialize, Serialize};

use crate::components::GamePosition;

use super::{object::Object, terrain::TerrainState, saves::User};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct World {
    pub players: Vec<(User, GamePosition)>,
    pub offline_players: Vec<(User, GamePosition)>,
    pub terrain_changes: Vec<((isize, isize), Vec<(usize, usize, TerrainState)>)>,
    pub objects: Vec<Object>
}

impl World {
    pub fn new() -> World {
        World {
            players: vec![],
            offline_players: vec![],
            terrain_changes: vec![],
            objects: vec![]
        }
    }
    pub fn clone_chunk(&mut self, chunk: (isize, isize)) -> Vec<(usize, usize, TerrainState)> {
        for (loc, data) in &self.terrain_changes {
            if loc == &chunk {
                return data.clone();
            }
        }
        return vec![];
    }
    pub fn _modify_tile(&mut self, chunk: (isize, isize), tile: (usize, usize), state: TerrainState) {
        let mut target_index = 0;
        let mut found_target = false;
        for (index, (loc, _data)) in self.terrain_changes.iter().enumerate() {
            if loc == &chunk {
                target_index = index;
                found_target = true;
            }
        }
        if found_target {
            for (index2, (posx, posy, _state)) in self.terrain_changes[target_index].1.iter().enumerate() {
                if posx == &tile.0 && posy == &tile.1 {
                    self.terrain_changes[target_index].1[index2].2 = state;
                    return;
                }
            }
            self.terrain_changes[target_index].1.push((tile.0, tile.1, state));
        }
        else {
            self.terrain_changes.push((chunk, vec![(tile.0, tile.1, state)]));
        }
    }
}
