use bevy::prelude::*;
use crate::{MapAssets, ldtk::{LDtkMap, load_level}};

pub fn load_title_screen_map(
    mut commands: Commands,
    mut maps: ResMut<Assets<LDtkMap>>,
    target_maps: Res<MapAssets>
) {
    let a = maps.get_mut(target_maps.player.clone()).unwrap();
    let level = a.get_level("title_screen");
    load_level(level, &mut commands);
}
