use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct World {
    pub players: Vec<Player>
}

impl World {
    pub fn new() -> World {
        World {
            players: vec![]
        }
    }
}