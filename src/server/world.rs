use crate::prelude::*;
use bevy::utils::HashMap;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct World {
    pub players: Vec<(User, Transform, PlayerData)>,
    pub offline_players: Vec<(User, Transform, PlayerData)>,
    /// All of the generated terrain in the world.
    /// (chunk coords, terrain data array)
    /// Each chunk is a 2d array of size `CHUNK_WIDTH` * `CHUNK_HEIGHT`, and starts in the logical
    /// top left.
    pub terrain: HashMap<(isize, isize), Vec<usize>>,
    pub objects: Vec<Object>,
    pub generated_objects: Vec<(isize, isize)>,
}

impl World {
    pub fn new() -> World {
        World {
            players: vec![],
            offline_players: vec![],
            terrain: default(),
            objects: vec![],
            generated_objects: vec![],
        }
    }
    pub fn get_or_gen(&mut self, chunk: (isize, isize)) -> Vec<usize> {
        if let Some(chunk_data) = self.terrain.get(&chunk) {
            chunk_data.clone()
        }
        else {
            self.generate_terrain(chunk);
            self.get_or_gen(chunk)
        }
    }
    fn generate_terrain(&mut self, chunk: (isize, isize)) {
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
        let layers = level
            .layer_instances
            .as_ref()
            .expect("FATAL: The LDtk option to save levels/layers seperately isn't supported.");
        let mut chunk_data = vec![];
        for layer in layers {
            if layer.layer_instance_type.as_str() == "IntGrid" {
                if layer.identifier == "Terrain" {
                    for pot_height_layer in 0..CHUNK_HEIGHT {
                        let height_layer = CHUNK_HEIGHT - 1 - pot_height_layer;
                        for width_layer in 0..CHUNK_WIDTH {
                            chunk_data.push((layer.int_grid_csv[width_layer + (height_layer * CHUNK_WIDTH)] - 1) as usize);
                        }
                    }
                }
            }
        }
        // check data
        if chunk_data.len() != CHUNK_SIZE {
            error!(
                "Chunk terrain data was improper ({} != {CHUNK_SIZE})",
                chunk_data.len()
            );
            error!("Chunk location: ({}, {})", chunk.0, chunk.1);
            panic!("{FATAL_ERROR}");
        }
        let mut final_data = vec![];
        for i in 0..chunk_data.len() {
            final_data.push(chunk_data[i]);
        }
        // save data
        self.terrain.insert(chunk, final_data);
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
        let layers = level
            .layer_instances
            .as_ref()
            .expect("FATAL: The LDtk option to save levels/layers seperately isn't supported.");
        let mut dupe_objects = vec![];
        for layer in layers {
            if layer.layer_instance_type.as_str() == "Entities" {
                for entity in &layer.entity_instances {
                    match entity.identifier.as_str() {
                        "Tree" => {
                            self.objects.push(Object {
                                pos: Transform::from_xyz(
                                    (-1920.0 / 2.0) + entity.px[0] as f32 + 32.0 + (1920.0 * chunk.0 as f32),
                                    (1080.0 / 2.0) - entity.px[1] as f32 - 32.0 + (1088.0 * chunk.1 as f32),
                                    0.0
                                ),
                                rep: ObjectType::Tree(3),
                                uuid: uuid::Uuid::parse_str(&entity.iid)
                                    .expect("FATAL: LDtk entity had an invalid UUID"),
                            });
                            dupe_objects.push(self.objects[self.objects.len() - 1].clone());
                        }
                        "Item" => {
                            for dataseg in &entity.field_instances {
                                if dataseg.identifier == "ItemName" {
                                    let item = Item::from_str(
                                        dataseg
                                            .value
                                            .as_ref()
                                            .expect("FATAL: LDtk entity of type Item had no ItemName")
                                            .as_str()
                                            .expect("FATAL: LDtk entity had a non-string ItemName"),
                                    );
                                    self.objects.push(Object {
                                        pos: Transform::from_xyz(
                                            (-1920.0 / 2.0) + entity.px[0] as f32 + 32.0 + (1920.0 * chunk.0 as f32),
                                            (1080.0 / 2.0) - entity.px[1] as f32 - 32.0 + (1088.0 * chunk.1 as f32),
                                            0.0
                                        ),
                                        rep: ObjectType::GroundItem(item),
                                        uuid: uuid::Uuid::parse_str(&entity.iid)
                                            .expect("FATAL: LDtk entity had an invalid UUID"),
                                    });
                                    dupe_objects.push(self.objects[self.objects.len() - 1].clone());
                                }
                            }
                        }
                        "NPC" => {
                            for dataseg in &entity.field_instances {
                                if dataseg.identifier == "NPCName" {
                                    let npc = Npc::from_name_str(
                                        dataseg
                                            .value
                                            .as_ref()
                                            .expect("FATAL: LDtk entity of type NPC had no NPCName")
                                            .as_str()
                                            .expect("FATAL: LDtk entity had a non-string NPCName"),
                                    );
                                    self.objects.push(Object {
                                        pos: Transform::from_xyz(
                                            (-1920.0 / 2.0) + entity.px[0] as f32 + 32.0 + (1920.0 * chunk.0 as f32),
                                            (1080.0 / 2.0) - entity.px[1] as f32 - 32.0 + (1088.0 * chunk.1 as f32),
                                            0.0
                                        ),
                                        rep: ObjectType::Npc(npc),
                                        uuid: uuid::Uuid::parse_str(&entity.iid)
                                            .expect("FATAL: LDtk entity had an invalid UUID"),
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
        }

        self.generated_objects.push(chunk);

        dupe_objects
    }
    /// Input tile coordinates are world aligned (+x right, +y up) starting in the logical bottom
    /// left
    pub fn modify_tile(&mut self, chunk: (isize, isize), tile: (usize, usize), state: usize) {
        let mut dta = self.get_or_gen(chunk);
        dta[tile.0 + (tile.1 * CHUNK_WIDTH)] = state;
        // TODO: mut access for reduced overhead
        self.terrain.insert(chunk, dta);
    }
}
