/*
 * FILE CREDIT
 * ===========
 * SIGNIFICANT PORTIONS OF THIS CODE ARE COPIED OR OTHERWISE MODIFIED FROM 
 * https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/ldtk/ldtk.rs
 * WHICH IS UNDER AN OPEN-SOURCE MIT LICENSE.
 */
use bevy::{
    utils::HashMap,
    reflect::TypeUuid,
    prelude::*,
    asset::{
        AssetLoader,
        LoadContext,
        BoxedFuture,
        LoadedAsset,
        AssetPath
    }
};

use crate::{
    assets::FontAssets,
    components::ldtk::{
        TileMarker,
        Tile
    },
    consts::{
        BACKGROUND,
        UI_TEXT
    },
    resources::ui::{
        UIManager,
        UIClickable,
        UIClickAction
    }
};

pub fn load_chunk(
    chunk: (isize, isize),
    map: &LDtkMap,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    fonts: FontAssets,
    commands: &mut Commands
) {
    // Create the level name from numbers.
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
    for level in &map.project.levels {
        if level.identifier == format!("Env_{}_{}", fmta, fmtb) {
            selected_level = Some(level);
        }
    }
    
    // Use the backup if this level doesn't exist.
    if selected_level.is_none() {
        for level in &map.project.levels {
            if level.identifier == "Env_NONE" {
                selected_level = Some(level);
            }
        }
    }

    // Take the level and load it!
    let level = selected_level.unwrap();
    let layers = level.layer_instances.as_ref().expect("FATAL: The LDtk option to save levels/layers seperately isn't supported.");
    for (lindex, layer) in layers.iter().rev().enumerate() {
        match layer.layer_instance_type.as_str() {
            "Tiles" => {
                let tileset_id = layer.tileset_def_uid.unwrap();
                let tileset = map.tilesets.get(&tileset_id).unwrap();
                let mut tileset_definition = None;
                for tileset in &map.project.defs.tilesets {
                    if tileset.uid == tileset_id {
                        tileset_definition = Some(tileset);
                    }
                }
                let tileset_definition = tileset_definition.unwrap();
                let texture_atlas = TextureAtlas::from_grid(
                    tileset.clone(),
                    Vec2::from((tileset_definition.tile_grid_size as f32, tileset_definition.tile_grid_size as f32)),
                    tileset_definition.c_hei as usize, tileset_definition.c_wid as usize
                );
                let atlas_handle = texture_atlases.add(texture_atlas);
                for tile in &layer.grid_tiles {
                    let tileset_tile_id = tile.t;
                    commands.spawn_bundle(SpriteSheetBundle {
                        transform: Transform::from_xyz(
                            (-1920.0 / 2.0) + tile.px[0] as f32 + 32.0 + (1920.0 * chunk.0 as f32),
                            (1080.0 / 2.0) - tile.px[1] as f32 - 32.0 + (1088.0 * chunk.1 as f32),
                            BACKGROUND + lindex as f32
                        ),
                        texture_atlas: atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(tileset_tile_id as usize),
                        ..Default::default()
                    }).insert(Tile {
                        chunk,
                        position: (
                            (tile.px[0] / 64) as usize,
                            17 - (tile.px[1] / 64) as usize
                        ),
                        sprite: (tileset_id as usize, tileset_tile_id as usize)
                    });
                }
                
            }
            "Entities" => {
                for entity in &layer.entity_instances {
                    match entity.identifier.as_str() {
                        "Text" => {
                            let mut text = String::new();
                            let mut font_size = 1.0;
                            for field in &entity.field_instances {
                                if field.identifier == "Text" {
                                    text = field.value.clone().unwrap().as_str().unwrap().to_string();
                                }
                                if field.identifier == "Font_Size" {
                                    font_size = field.value.clone().unwrap().as_f64().unwrap();
                                }
                            }
                            commands.spawn_bundle(Text2dBundle {
                                transform: Transform::from_xyz(
                                    (-1920.0 / 2.0) + entity.px[0] as f32 + (entity.width as f32 / 2.0),
                                    (1080.0 / 2.0) - entity.px[1] as f32 - (entity.height as f32 / 2.0),
                                    UI_TEXT
                                ),
                                text: Text {
                                    alignment: TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center
                                    },
                                    sections: vec![
                                        TextSection {
                                            value: text,
                                            style: TextStyle {
                                                font: fonts.simvoni.clone(),
                                                font_size: font_size as f32,
                                                color: Color::BLACK
                                            }
                                        }
                                    ]
                                },
                                ..Default::default()
                            }).insert(TileMarker {});
                        }
                        "LoadLevel" => {
                            println!("WARNING: LDtk entity LoadLevel does not work for ENV_ levels.");
                        }
                        ei => {
                            println!("WARNING: LDtk file had an entity named {}, which isn't known or supported.", ei);
                        }
                    }
                }
            }
            it => {
                panic!("FATAL: LDtk file had an invalid instance type {}.", it)
            }
        }
    }
}

pub fn load_level(
    unloads: Query<Entity, With<TileMarker>>,
    level: &ldtk_rust::Level,
    map: &LDtkMap,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    fonts: FontAssets,
    mut uimanager: ResMut<UIManager>,
    commands: &mut Commands
) {
    unloads.for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
    let layers = level.layer_instances.as_ref().expect("FATAL: The LDtk option to save levels/layers seperately isn't supported.");
    for layer in layers.iter().rev() {
        match layer.layer_instance_type.as_str() {
            "Tiles" => {
                let tileset_id = layer.tileset_def_uid.unwrap();
                let tileset = map.tilesets.get(&tileset_id).unwrap();
                let mut tileset_definition = None;
                for tileset in &map.project.defs.tilesets {
                    if tileset.uid == tileset_id {
                        tileset_definition = Some(tileset);
                    }
                }
                let tileset_definition = tileset_definition.unwrap();
                let texture_atlas = TextureAtlas::from_grid(
                    tileset.clone(),
                    Vec2::from((tileset_definition.tile_grid_size as f32, tileset_definition.tile_grid_size as f32)),
                    tileset_definition.c_hei as usize, tileset_definition.c_wid as usize
                );
                let atlas_handle = texture_atlases.add(texture_atlas);
                for tile in &layer.grid_tiles {
                    let tileset_tile_id = tile.t;
                    commands.spawn_bundle(SpriteSheetBundle {
                        transform: Transform::from_xyz(
                            (-1920.0 / 2.0) + tile.px[0] as f32 + 32.0,
                            (1080.0 / 2.0) - tile.px[1] as f32 - 32.0,
                            BACKGROUND),
                        texture_atlas: atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(tileset_tile_id as usize),
                        ..Default::default()
                    }).insert(TileMarker {});
                }
                
            }
            "Entities" => {
                for entity in &layer.entity_instances {
                    match entity.identifier.as_str() {
                        "Text" => {
                            let mut text = String::new();
                            let mut font_size = 1.0;
                            for field in &entity.field_instances {
                                if field.identifier == "Text" {
                                    text = field.value.clone().unwrap().as_str().unwrap().to_string();
                                }
                                if field.identifier == "Font_Size" {
                                    font_size = field.value.clone().unwrap().as_f64().unwrap();
                                }
                            }
                            commands.spawn_bundle(Text2dBundle {
                                transform: Transform::from_xyz(
                                    (-1920.0 / 2.0) + entity.px[0] as f32 + (entity.width as f32 / 2.0),
                                    (1080.0 / 2.0) - entity.px[1] as f32 - (entity.height as f32 / 2.0),
                                    UI_TEXT
                                ),
                                text: Text {
                                    alignment: TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center
                                    },
                                    sections: vec![
                                        TextSection {
                                            value: text,
                                            style: TextStyle {
                                                font: fonts.simvoni.clone(),
                                                font_size: font_size as f32,
                                                color: Color::BLACK
                                            }
                                        }
                                    ]
                                },
                                ..Default::default()
                            }).insert(TileMarker {});
                        }
                        "LoadLevel" => {
                            let mut level = String::new();
                            for field in &entity.field_instances {
                                if field.identifier == "LoadableLevel" {
                                    level = field.value.clone().unwrap().as_str().unwrap().to_string();
                                }
                            }
                            uimanager.add_ui(UIClickable {
                                action: UIClickAction::ChangeScene(level),
                                removed_on_use: false,
                                location: (
                                    (-1920.0 / 2.0) + entity.px[0] as f32,
                                    (1080.0 / 2.0) - entity.px[1] as f32
                                ),
                                size: (entity.width as f32, entity.height as f32)
                            });
                        }
                        "GameplayTrigger" => {
                            let mut action = String::new();
                            for field in &entity.field_instances {
                                if field.identifier == "Trigger" {
                                    action = field.value.clone().unwrap().as_str().unwrap().to_string();
                                }
                            }

                            uimanager.add_ui(UIClickable {
                                action: UIClickAction::GameplayTrigger(action),
                                location: (
                                    (-1920.0 / 2.0) + entity.px[0] as f32,
                                    (1080.0 / 2.0) - entity.px[1] as f32
                                ),
                                size: (entity.width as f32, entity.height as f32),
                                removed_on_use: true 
                            });
                        }
                        ei => {
                            println!("WARNING: LDtk file had an entity named {}, which isn't known or supported.", ei);
                        }
                    }
                }
            }
            it => {
                panic!("FATAL: LDtk file had an invalid instance type {}.", it)
            }
        }
    }
}

#[derive(Default)]
pub struct LDtkPlugin;

impl Plugin for LDtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<LDtkMap>()
            .add_asset_loader(LDtkLoader);
    }
}

#[derive(TypeUuid)]
#[uuid = "e51081d0-6168-4881-a1c6-4249b2000d7f"]
pub struct LDtkMap {
    pub project: ldtk_rust::Project,
    pub tilesets: HashMap<i64, Handle<Image>>
}

impl LDtkMap {
    pub fn get_level(&self, identifier: &str) -> &ldtk_rust::Level {
        for level in &self.project.levels {
            if level.identifier == identifier {
                return level;
            }
        }
        panic!("no level exists for identifier {}!", identifier);
    }
}

pub struct LDtkLoader;

impl AssetLoader for LDtkLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let project: ldtk_rust::Project = serde_json::from_slice(bytes)?;
            
            let mut dependencies: Vec<(i64, AssetPath)> = vec![];
            for tileset in project.defs.tilesets.iter() {
                if let Some(path) = &tileset.rel_path {
                    dependencies.push(
                        (
                            tileset.uid,
                            load_context
                                .path()
                                .parent()
                                .unwrap()
                                .join(path)
                                .into()
                        )
                    );
                }
            }

            let loaded_asset = LoadedAsset::new(LDtkMap {
                project,
                tilesets: dependencies
                    .iter()
                    .map(|dep| (dep.0, load_context.get_handle(dep.1.clone())))
                    .collect(),
            });
            load_context.set_default_asset(
                loaded_asset.with_dependencies(dependencies.iter().map(|x| x.1.clone()).collect()),
            );
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["ldtk"];
        EXTENSIONS
    }
}
