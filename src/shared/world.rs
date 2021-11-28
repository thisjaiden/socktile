use serde::{Deserialize, Serialize};

use super::{object::Object, player::Player, terrain::TerrainState};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct World {
    pub players: Vec<Player>,
    pub offline_players: Vec<Player>,
    // reminder: all chunks are 32x32, with world coordinates starting in the center.
    // all terrain objects are 32x32 FOR NOW
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
    pub fn clone_chunk(&mut self, chunk: (usize, usize)) -> Vec<TerrainState> {
        for (loc, data) in &self.terrain_chunks {
            if loc == &chunk {
                return data.clone();
            }
        }
        self.generate_chunk(chunk);
        return self.terrain_chunks.last().unwrap().1.clone();
    }
    pub fn modify_chunk(&mut self, chunk: (usize, usize), modify: &mut dyn FnMut(&mut Vec<TerrainState>)) {
        let mut index: usize = 0;
        let mut needs_generation = true;
        for (loc, _) in &self.terrain_chunks {
            if loc == &chunk {
                needs_generation = false;
                break;
            }
            index += 1;
        }
        if needs_generation {
            index = self.terrain_chunks.len() - 1;
            self.generate_chunk(chunk)
        }
        modify(&mut self.terrain_chunks[index].1);
    }
    pub fn generate_chunk(&mut self, chunk: (usize, usize)) {
        self.terrain_chunks.push((
            chunk,
            vec![TerrainState::EmptyWater; 1024]
        ));
    }
}
