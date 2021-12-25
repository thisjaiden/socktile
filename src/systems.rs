pub mod netty;
pub mod visual;
pub mod text_box;
pub mod cursor;

use bevy::prelude::*;

use crate::{components::{TextBox, ldtk::TileMarker}, resources::{Netty, ui::UIManager}, shared::{netty::Packet, saves::{User, save_user}}, GameState, ldtk::{load_level, LDtkMap}, FontAssets, MapAssets};

pub fn user_creation(
    mut commands: Commands,
    mut tb: ResMut<crate::resources::TextBox>,
    mut tb_q: Query<(Entity, &mut Text), With<TextBox>>,
    mut netty: ResMut<Netty>,
    mut state: ResMut<State<GameState>>,
    unloads: Query<Entity, With<TileMarker>>,
    mut maps: ResMut<Assets<LDtkMap>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    target_maps: Res<MapAssets>,
    font_assets: Res<FontAssets>,
    uiman: ResMut<UIManager>
) {
    let (entity, mut text) = tb_q.single_mut().unwrap();
    text.sections[0].value = tb.grab_buffer() + "#0000";
    if tb.grab_buffer().contains('#') || tb.grab_buffer().is_empty() {
        text.sections[0].style.color = Color::RED;
    }
    else {
        text.sections[0].style.color = Color::BLACK;
        if tb.grab_buffer().contains('\n') {
            let mut mode = tb.grab_buffer();
            mode = String::from(mode.trim_end());
            mode = String::from(mode.trim_end_matches('\n'));
            netty.say(Packet::CreateUser(User {
                username: mode.clone(),
                tag: 0
            }));
            save_user(User {
                username: mode,
                tag: 0
            });
            tb.clear_buffer();
            state.replace(GameState::TitleScreen).unwrap();
            commands.entity(entity).despawn_recursive();
            let a = maps.get_mut(target_maps.player.clone()).unwrap();
            let level = a.get_level("Title_screen");
            load_level(unloads, level, a, texture_atlases, font_assets.clone(), uiman, &mut commands);
        }
    }
}
