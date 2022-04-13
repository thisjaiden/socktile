use serde::{Deserialize, Serialize};

use crate::components::GamePosition;

use super::{object::{Object, ObjectType}, terrain::TerrainState, saves::User, player::PlayerData};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct World {
    pub players: Vec<(User, GamePosition, PlayerData)>,
    pub offline_players: Vec<(User, GamePosition, PlayerData)>,
    pub terrain_changes: Vec<((isize, isize), Vec<(usize, usize, TerrainState)>)>,
    pub objects: Vec<Object>,
    pub generated_objects: Vec<(isize, isize)>
}

impl World {
    pub fn new() -> World {
        World {
            players: vec![],
            offline_players: vec![],
            terrain_changes: vec![],
            objects: vec![],
            generated_objects: vec![]
        }
    }
    pub fn try_generating_objects(&mut self, chunk: (isize, isize)) -> Vec<Object> {
        if self.generated_objects.contains(&chunk) {
            // no chunk generation needed
            return vec![];
        }
        let project: ldtk_rust::Project = serde_json::from_slice(include_bytes!("..\\..\\assets\\core.ldtk"))
            .expect("FATAL: Invalid LDTK map for server executable, this is an unrepairable error.");
        
        // find the level name from a chunk
        let fmta = if chunk.0.is_negative() {
            format!("M{}", -chunk.0)
        }
        else {
            format!("{}", chunk.0)
        };
        let fmtb = if chunk.1.is_negative() {
            format!("M{}", -chunk.1)
        }
        else {
            format!("{}", chunk.1)
        };

        // Search for the level in the world.
        let mut selected_level = None;
        for level in &project.levels {
            if level.identifier == format!("Env_{}_{}", fmta, fmtb) {
                selected_level = Some(level);
            }
        }
        
        // Use the backup if this level doesn't exist.
        if selected_level.is_none() {
            for level in &project.levels {
                if level.identifier == "Env_NONE" {
                    selected_level = Some(level);
                }
            }
        }

        let level = selected_level.expect("FATAL: LDTK file missing required backup environment");
        let layers = level.layer_instances.as_ref().expect("FATAL: The LDtk option to save levels/layers seperately isn't supported.");
        let mut dupe_objects = vec![];
        for layer in layers {
            match layer.layer_instance_type.as_str() {
                "Tiles" => {
                    // ignored
                }
                "Entities" => {
                    for entity in &layer.entity_instances {
                        match entity.identifier.as_str() {
                            "Tree" => {
                                self.objects.push(Object {
                                    pos: GamePosition {
                                        x: (-1920.0 / 2.0) + entity.px[0] as f64 + 32.0 + (1920.0 * chunk.0 as f64),
                                        y: (1080.0 / 2.0) - entity.px[1] as f64 - 32.0 + (1088.0 * chunk.1 as f64)
                                    },
                                    rep: ObjectType::Tree,
                                    uuid: uuid::Uuid::parse_str(&entity.iid).expect("FATAL: LDtk entity had an invalid UUID")
                                });
                                dupe_objects.push(self.objects[self.objects.len() - 1].clone());
                            }
                            _ => {
                                // ignored or otherwise unknown.
                            }
                        }
                    }
                }
                it => {
                    panic!("FATAL: LDtk file had an invalid instance type {it}.")
                }
            }
        }

        self.generated_objects.push(chunk);

        return dupe_objects;
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
