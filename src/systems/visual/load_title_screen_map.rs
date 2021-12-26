use bevy::prelude::*;
use crate::{MapAssets, ldtk::{LDtkMap, load_level}, components::ldtk::TileMarker, shared::{saves::user, netty::Packet}, GameState, FontAssets, resources::{ui::UIManager, Netty}};

pub fn load_title_screen_map(
    mut commands: Commands,
    unloads: Query<Entity, With<TileMarker>>,
    mut maps: ResMut<Assets<LDtkMap>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    target_maps: Res<MapAssets>,
    mut state: ResMut<State<GameState>>,
    font_assets: Res<FontAssets>,
    uiman: ResMut<UIManager>,
    mut netty: ResMut<Netty>
) {
    if user().is_some() {
        let a = maps.get_mut(target_maps.player.clone()).unwrap();
        let level = a.get_level("Title_screen");
        load_level(unloads, level, a, texture_atlases, font_assets.clone(), uiman, &mut commands);
        netty.say(Packet::UserPresence(user().unwrap()));
    }
    else {
        state.set(GameState::MakeUser).unwrap();
    }
}
