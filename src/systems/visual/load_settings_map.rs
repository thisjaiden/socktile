use bevy::prelude::*;
use crate::{assets::{MapAssets, FontAssets}, ldtk::{LDtkMap, load_level}, components::ldtk::TileMarker, resources::ui::UIManager};

pub fn load_settings_map(
    mut commands: Commands,
    unloads: Query<Entity, With<TileMarker>>,
    mut maps: ResMut<Assets<LDtkMap>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    target_maps: Res<MapAssets>,
    font_assets: Res<FontAssets>,
    uiman: ResMut<UIManager>
) {
    let a = maps.get_mut(target_maps.core.clone()).unwrap();
    let level = a.get_level("Settings");
    load_level(unloads, level, a, texture_atlases, font_assets.clone(), uiman, &mut commands);
}
