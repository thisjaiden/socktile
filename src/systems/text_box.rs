use bevy::prelude::*;

use crate::{components::{TextBox, ldtk::{TileMarker, PlayerMarker}}, resources::{Netty, ui::UIManager, Disk}, shared::{netty::Packet, saves::User}, GameState, ldtk::{load_level, LDtkMap}, assets::{FontAssets, MapAssets, AnimatorAssets}, consts::{PLAYER_CHARACTERS, UI_TEXT}};

pub fn text_input(
    mut tb: ResMut<crate::resources::TextBox>,
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>
) {
    for char in char_evr.iter() {
        tb.update_buffer(char.char);
        if char.char == '\r' {
            tb.eat_buffer();
            tb.update_buffer('\n');
        }
    }
    if keys.just_pressed(KeyCode::Back) {
        tb.eat_buffer();
    }
}

pub fn text_backspace(
    mut tb: ResMut<crate::resources::TextBox>,
    keys: Res<Input<KeyCode>>
) {
    if keys.just_pressed(KeyCode::Back) {
        tb.eat_buffer();
    }
}

pub fn user_creation(
    mut commands: Commands,
    mut tb: ResMut<crate::resources::TextBox>,
    mut tb_q: Query<(Entity, &mut Text), With<TextBox>>,
    mut netty: ResMut<Netty>,
    mut state: ResMut<State<GameState>>,
    mut disk: ResMut<Disk>,
    unloads: Query<Entity, With<TileMarker>>,
    mut maps: ResMut<Assets<LDtkMap>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    target_maps: Res<MapAssets>,
    font_assets: Res<FontAssets>,
    uiman: ResMut<UIManager>
) {
    let (entity, mut text) = tb_q.single_mut();
    text.sections[0].value = tb.grab_buffer() + "";
    if tb.grab_buffer().contains('#') {
        text.sections[0].style.color = Color::RED;
        text.sections[1].value = String::from("\nUsernames can't contain hashtags.");
    }
    else if tb.grab_buffer().contains('/') || tb.grab_buffer().contains('\\') {
        text.sections[0].style.color = Color::RED;
        text.sections[1].value = String::from("\nUsernames can't contain slashes.");
    }
    else if tb.grab_buffer().starts_with(' ') {
        text.sections[0].style.color = Color::RED;
        text.sections[1].value = String::from("\nUsernames can't start with a space.");
    }
    else if tb.grab_buffer().ends_with(' ') {
        text.sections[0].style.color = Color::RED;
        text.sections[1].value = String::from("\nUsernames can't end with a space.");
    }
    else if !tb.grab_buffer().is_empty() {
        // Reset text to normal if all is okay
        text.sections[1].value = String::from("\n");
        text.sections[0].style.color = Color::BLACK;
        // Warnings for inconvenient but not disallowed names
        if tb.grab_buffer().chars().count() > 20 {
            text.sections[0].style.color = Color::ORANGE;
            text.sections[1].value = String::from("\nUsernames over 20 characters may be inconvenient.");
        }
        else if tb.grab_buffer().contains("  ") {
            text.sections[0].style.color = Color::ORANGE;
            text.sections[1].value = String::from("\nUsernames with multiple spaces in a row may be inconvenient.");
        }
        if tb.grab_buffer().contains('\n') {
            let mut mode = tb.grab_buffer();
            mode = String::from(mode.trim_end());
            mode = String::from(mode.trim_end_matches('\n'));
            netty.say(Packet::CreateUser(User {
                username: mode.clone(),
                tag: 0
            }));
            while !disk.update_user(User {
                username: mode.clone(),
                tag: 0
            }) {}
            tb.clear_buffer();
            state.replace(GameState::TitleScreen).unwrap();
            commands.entity(entity).despawn_recursive();
            let a = maps.get_mut(target_maps.core.clone()).unwrap();
            let level = a.get_level("Title_screen");
            load_level(unloads, level, a, texture_atlases, font_assets.clone(), uiman, &mut commands);
        }
    }
}

pub fn game_creation(
    mut commands: Commands,
    mut tb: ResMut<crate::resources::TextBox>,
    mut tb_q: Query<(Entity, &mut Text), With<TextBox>>,
    mut netty: ResMut<Netty>,
    mut state: ResMut<State<GameState>>,
    disk: Res<Disk>,
    materials: Res<AnimatorAssets>,
    unloads: Query<Entity, With<TileMarker>>
) {
    let (entity, mut text) = tb_q.single_mut();
    text.sections[0].value = tb.grab_buffer();
    if tb.grab_buffer().contains('#') || tb.grab_buffer().is_empty() {
        text.sections[0].style.color = Color::RED;
    }
    else {
        text.sections[0].style.color = Color::BLACK;
        if tb.grab_buffer().contains('\n') {
            let mut mode = tb.grab_buffer();
            mode = String::from(mode.trim_end());
            mode = String::from(mode.trim_end_matches('\n'));
            netty.say(Packet::CreateWorld(mode));
            tb.clear_buffer();
            state.replace(GameState::Play).unwrap();
            commands.entity(entity).despawn_recursive();
            commands.spawn_bundle(SpriteBundle {
                texture: materials.placeholder.clone(),
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    PLAYER_CHARACTERS
                ),
                ..Default::default()
            }).insert(PlayerMarker { user: disk.user().unwrap(), isme: true });

            unloads.for_each(|e| {
                commands.entity(e).despawn_recursive();
            });
        }
    }
}

pub fn game_creation_once(
    mut commands: Commands,
    font_assets: Res<FontAssets>
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: String::new(),
                    style: TextStyle {
                        font: font_assets.simvoni.clone(),
                        font_size: 35.0,
                        color: Color::BLACK
                    }
                }
            ],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center
            }
        },
        transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
        ..Default::default()
    }).insert(TextBox {});
}
