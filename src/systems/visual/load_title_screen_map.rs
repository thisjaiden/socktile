use bevy::prelude::*;
use crate::{MapAssets, ldtk::{LDtkMap, load_level}};

pub fn load_title_screen_map(
    mut commands: Commands,
    mut maps: ResMut<Assets<LDtkMap>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    target_maps: Res<MapAssets>
) {
    let a = maps.get_mut(target_maps.player.clone()).unwrap();
    let level = a.get_level("Title_screen");
    load_level(level, a, texture_atlases, &mut commands);
}
