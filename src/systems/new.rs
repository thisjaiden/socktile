use bevy::prelude::*;

use crate::{components::{NewManager, PlayManager}, layers::{UI_TEXT}, resources::{AssetHandles, GameState, Netty, TextBox}, shared::{netty::Packet}};

pub fn new(
    mut commands: Commands,
    state: Res<GameState>,
    mut handles: ResMut<AssetHandles>
) {
    if state.eq(&GameState::New) && state.is_changed() {
        let entity_ids = vec![
            commands.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    "Name your world.",
                    TextStyle {
                        font: handles.get_font("KreativeSquare.ttf"),
                        font_size: 34.0,
                        color: Color::BLACK
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center
                    }
                ),
                transform: Transform::from_xyz(0.0, 40.0, UI_TEXT),
                ..Default::default()
            }).id(),
            commands.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: handles.get_font("KreativeSquare.ttf"),
                        font_size: 34.0,
                        color: Color::BLACK
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center
                    }
                ),
                transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
                ..Default::default()
            }).insert(crate::components::TextBox {}).id()
        ];

        commands.spawn().insert(
            NewManager::new(entity_ids)
        );
    }
}

pub fn new_ui(
    _commands: Commands,
    mut tb: ResMut<TextBox>,
    manager: Query<&mut NewManager>,
    tb_q: Query<&mut Text, With<crate::components::TextBox>>,
    state: Res<GameState>,
    mut netty: ResMut<Netty>
) {
    if state.eq(&GameState::New) {
        tb_q.for_each_mut(|mut text| {
            text.sections[0].value = tb.grab_buffer();
            if tb.grab_buffer().is_empty() {
                text.sections[0].style.color = Color::RED;
            }
            else {
                text.sections[0].style.color = Color::BLACK;
                if tb.grab_buffer().contains('\n') {
                    manager.for_each_mut(|mut state_man| {
                        if !state_man.is_waiting() {
                            let mut mode = tb.grab_buffer();
                            mode = String::from(mode.trim_end());
                            mode = String::from(mode.trim_end_matches('\n'));
                            netty.say(Packet::CreateWorld(mode));
                            state_man.net_mode();
                            tb.clear_buffer();
                        }
                    });
                }
            }
        });
    }
}

pub fn new_exit(
    mut commands: Commands,
    manager: Query<&mut NewManager>,
    mut state: ResMut<GameState>,
) {
    manager.for_each_mut(|mut manager| {
        if manager.swap_time() {
            // TODO
            manager.disassemble(&mut commands);
            commands.spawn().insert(PlayManager::new());
            state.change_state(GameState::Play);
        }
    });
}
