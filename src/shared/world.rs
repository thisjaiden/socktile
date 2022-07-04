use serde::{Deserialize, Serialize};

use bevy::prelude::*;

use crate::{components::GamePosition, server::npc::NPC, consts::{FATAL_ERROR, CHUNK_WIDTH}};
use super::{object::{Object, ObjectType}, saves::User, player::{PlayerData, Item}};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct World {
    pub players: Vec<(User, GamePosition, PlayerData)>,
    pub offline_players: Vec<(User, GamePosition, PlayerData)>,
    /// All of the generated terrain in the world.
    /// (chunk coords, terrain data array)
    pub terrain: Vec<((isize, isize), Vec<usize>)>,
    pub objects: Vec<Object>,
    pub generated_objects: Vec<(isize, isize)>
}

impl World {
    pub fn new() -> World {
        World {
            players: vec![],
            offline_players: vec![],
            terrain: vec![],
            objects: vec![],
            generated_objects: vec![]
        }
    }
    pub fn try_generating_objects(&mut self, chunk: (isize, isize)) -> Vec<Object> {
        if self.generated_objects.contains(&chunk) {
            // no chunk generation needed
            return vec![];
        }
        let project: ldtk_rust::Project = serde_json::from_slice(include_bytes!("../../assets/core.ldtk"))
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
                            "Item" => {
                                for dataseg in &entity.field_instances {
                                    if dataseg.identifier == "ItemName" {
                                        let item = Item::from_str(dataseg.value.as_ref()
                                            .expect("FATAL: LDtk entity of type Item had no ItemName")
                                            .as_str().expect("FATAL: LDtk entity had a non-string ItemName")
                                        );
                                        self.objects.push(Object {
                                            pos: GamePosition {
                                                x: (-1920.0 / 2.0) + entity.px[0] as f64 + 32.0 + (1920.0 * chunk.0 as f64),
                                                y: (1080.0 / 2.0) - entity.px[1] as f64 - 32.0 + (1088.0 * chunk.1 as f64)
                                            },
                                            rep: ObjectType::GroundItem(item),
                                            uuid: uuid::Uuid::parse_str(&entity.iid).expect("FATAL: LDtk entity had an invalid UUID")
                                        });
                                        dupe_objects.push(self.objects[self.objects.len() - 1].clone());
                                    }
                                }
                            }
                            "NPC" => {
                                for dataseg in &entity.field_instances {
                                    if dataseg.identifier == "NPCName" {
                                        let npc = NPC::from_name_str(dataseg.value.as_ref()
                                            .expect("FATAL: LDtk entity of type NPC had no NPCName")
                                            .as_str().expect("FATAL: LDtk entity had a non-string NPCName")
                                        );
                                        self.objects.push(Object {
                                            pos: GamePosition {
                                                x: (-1920.0 / 2.0) + entity.px[0] as f64 + 32.0 + (1920.0 * chunk.0 as f64),
                                                y: (1080.0 / 2.0) - entity.px[1] as f64 - 32.0 + (1088.0 * chunk.1 as f64)
                                            },
                                            rep: ObjectType::NPC(npc),
                                            uuid: uuid::Uuid::parse_str(&entity.iid).expect("FATAL: LDtk entity had an invalid UUID")
                                        });
                                        dupe_objects.push(self.objects[self.objects.len() - 1].clone());
                                    }
                                }
                            }
                            _ => {
                                // ignored or otherwise unknown.
                            }
                        }
                    }
                }
                invalid_instance => {
                    error!("LDtk file had an invalid instance type {invalid_instance}.");
                    panic!("{FATAL_ERROR}");
                }
            }
        }

        self.generated_objects.push(chunk);

        dupe_objects
    }
    pub fn clone_chunk(&mut self, chunk: (isize, isize)) -> Vec<usize> {
        for (loc, data) in &self.terrain {
            if loc == &chunk {
                return data.clone();
            }
        }
        warn!("Chunk data was requested for a chunk with nothing generated");
        return vec![];
    }
    pub fn _modify_tile(&mut self, chunk: (isize, isize), tile: (usize, usize), state: usize) {
        for (index, (loc, _data)) in self.terrain.iter().enumerate() {
            if loc == &chunk {
                self.terrain[index].1[tile.0 + tile.1 * CHUNK_WIDTH] = state;
                return;
            }
        }
        warn!("A change was requested for a tile in an ungenerated chunk. TODO: generate said chunk");
    }
}
