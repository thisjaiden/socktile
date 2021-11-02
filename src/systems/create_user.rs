use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::{components::CreateUserManager, layers::UI_TEXT, resources::{AssetHandles, GameState, TextBox}, shared::{netty::Packet, saves::{User, save_user}}};

pub fn create_user(
    mut tb: ResMut<TextBox>,
    mut commands: Commands,
    state: Res<GameState>,
    mut handles: ResMut<AssetHandles>,
) {
    if (state.eq(&GameState::CreateUser) || state.eq(&GameState::CreateUserB)) && state.is_changed() {
        tb.clear_buffer();
        let eids = vec![
            commands.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    "Please choose a username.",
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
                    "This can't be changed later.",
                    TextStyle {
                        font: handles.get_font("KreativeSquare.ttf"),
                        font_size: 25.0,
                        color: Color::BLACK
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center
                    }
                ),
                transform: Transform::from_xyz(0.0, -40.0, UI_TEXT),
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
        let recv = Arc::new(Mutex::new(vec![]));
        let send = Arc::new(Mutex::new(vec![]));
        if state.eq(&GameState::CreateUser) {
            commands.spawn().insert(CreateUserManager::new(eids, recv, send));
        }
        else {
            commands.spawn().insert(CreateUserManager::new_b(eids, recv, send));
        }
    }
}

pub fn create_user_ui(
    mut commands: Commands,
    mut tb: ResMut<TextBox>,
    manager: Query<&mut CreateUserManager>,
    tb_q: Query<&mut Text, With<crate::components::TextBox>>,
    mut state: ResMut<GameState>,
) {
    if state.eq(&GameState::CreateUser) || state.eq(&GameState::CreateUserB) {
        tb_q.for_each_mut(|mut text| {
            text.sections[0].value = tb.grab_buffer() + "#0000";
            if tb.grab_buffer().contains('#') || tb.grab_buffer().is_empty() {
                text.sections[0].style.color = Color::RED;
            }
            else {
                text.sections[0].style.color = Color::BLACK;
                if tb.grab_buffer().contains('\n') {
                    manager.for_each_mut(|mut state_man| {
                        let out = state_man.grab_out();
                        let mut mode = tb.grab_buffer();
                        mode = String::from(mode.trim_end());
                        mode = String::from(mode.trim_end_matches("\n"));
                        let mut o_acc = out.lock().unwrap();
                        o_acc.push(Packet::CreateProfile(User {
                            username: mode.clone(),
                            tag: 0
                        }));
                        drop(o_acc);
                        save_user(User {
                            username: mode,
                            tag: 0
                        });
                        state.change_state(state_man.grab_previous_state());
                        state_man.disassemble(&mut commands);
                    });
                }
            }
        });
    }
}
