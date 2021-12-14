/*
 * FILE CREDIT
 * ===========
 * SIGNIFICANT PORTIONS OF THIS CODE ARE COPIED OR OTHERWISE MODIFIED FROM 
 * https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/ldtk/ldtk.rs
 * WHICH IS UNDER AN OPEN-SOURCE MIT LICENSE.
 */
use bevy::asset::{AssetLoader, LoadContext, BoxedFuture, LoadedAsset, AssetPath};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use std::collections::HashMap;

use crate::layers::BACKGROUND;

pub fn load_level(
    level: &ldtk_rust::Level,
    map: &LDtkMap,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands
) {
    let layers = level.layer_instances.as_ref().expect("LDTK: SAVE LEVELS/LAYERS SEPERATELY IS **NOT** SUPPORTED!");
    for layer in layers {
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
                            tile.px[0] as f32 + level.world_x as f32 - (1920.0 / 2.0) + 32.0,
                            -tile.px[1] as f32 + (1080.0 / 2.0) - 32.0,
                            BACKGROUND),
                        texture_atlas: atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(tileset_tile_id as u32),
                        ..Default::default()
                    });
                }
                
            }
            it => {
                panic!("LDTK: INVALID INSTANCE TYPE {}.", it)
            }
        }
    }
}

#[derive(Default)]
pub struct LDtkPlugin;

impl Plugin for LDtkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<LDtkMap>()
            .add_asset_loader(LDtkLoader);
    }
}

#[derive(TypeUuid)]
#[uuid = "e51081d0-6168-4881-a1c6-4249b2000d7f"]
pub struct LDtkMap {
    pub project: ldtk_rust::Project,
    pub tilesets: HashMap<i64, Handle<Texture>>
}

impl LDtkMap {
    pub fn get_level(&self, identifier: &str) -> &ldtk_rust::Level {
        for level in &self.project.levels {
            if level.identifier == identifier {
                return level.clone();
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
            let dependencies: Vec<(i64, AssetPath)> = project
                .defs
                .tilesets
                .iter()
                .map(|tileset| {
                    (
                        tileset.uid,
                        load_context
                            .path()
                            .parent()
                            .unwrap()
                            .join(tileset.rel_path.clone())
                            .into(),
                    )
                })
                .collect();

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
