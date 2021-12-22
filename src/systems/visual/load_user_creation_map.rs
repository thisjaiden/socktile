use bevy::prelude::*;
use crate::{MapAssets, ldtk::{LDtkMap, load_level}, components::ldtk::TileMarker, FontAssets, resources::ui::UIManager};

pub fn load_user_creation_map(
    mut commands: Commands,
    unloads: Query<Entity, With<TileMarker>>,
    mut maps: ResMut<Assets<LDtkMap>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    target_maps: Res<MapAssets>,
    font_assets: Res<FontAssets>,
    uiman: ResMut<UIManager>
) {
    let a = maps.get_mut(target_maps.player.clone()).unwrap();
    let level = a.get_level("Create_user");
    load_level(unloads, level, a, texture_atlases, font_assets, uiman, &mut commands);
}
