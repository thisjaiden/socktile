use bevy::prelude::*;

use crate::{components::CreateUserManager, layers::UI_TEXT, resources::{AssetHandles, GameState, Netty, TextBox}, shared::{netty::Packet, saves::{User, save_user}}};

pub fn create_user(
    mut tb: ResMut<TextBox>,
    mut commands: Commands,
    state: Res<GameState>,
    mut handles: ResMut<AssetHandles>,
) {
    if state.eq(&GameState::CreateUser) && state.is_changed() {
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
        if state.eq(&GameState::CreateUser) {
            commands.spawn().insert(CreateUserManager::new(eids));
        }
        else {
            commands.spawn().insert(CreateUserManager::new_b(eids));
        }
    }
}

pub fn create_user_ui(
    mut commands: Commands,
    mut tb: ResMut<TextBox>,
    manager: Query<&mut CreateUserManager>,
    tb_q: Query<&mut Text, With<crate::components::TextBox>>,
    mut state: ResMut<GameState>,
    mut netty: ResMut<Netty>
) {
    if state.eq(&GameState::CreateUser) {
        tb_q.for_each_mut(|mut text| {
            text.sections[0].value = tb.grab_buffer() + "#0000";
            if tb.grab_buffer().contains('#') || tb.grab_buffer().is_empty() {
                text.sections[0].style.color = Color::RED;
            }
            else {
                text.sections[0].style.color = Color::BLACK;
                if tb.grab_buffer().contains('\n') {
                    manager.for_each_mut(|mut state_man| {
                        let mut mode = tb.grab_buffer();
                        mode = String::from(mode.trim_end());
                        mode = String::from(mode.trim_end_matches('\n'));
                        netty.say(Packet::CreateProfile(User {
                            username: mode.clone(),
                            tag: 0
                        }));
                        save_user(User {
                            username: mode,
                            tag: 0
                        });
                        tb.clear_buffer();
                        state.change_state(GameState::TitleScreen);
                        state_man.disassemble(&mut commands);
                    });
                }
            }
        });
    }
}
