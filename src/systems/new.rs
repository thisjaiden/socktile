use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::{client::core::startup, components::{CreateUserManager, CursorMarker, NewManager, PlayManager}, layers::{CURSOR, UI_TEXT}, resources::{AssetHandles, GameState, TextBox}, shared::{netty::Packet, saves::user}};

pub fn new(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    mut handles: ResMut<AssetHandles>,
    old_manager: Query<&mut CreateUserManager>,
) {
    if state.eq(&GameState::New) && state.is_changed() {
        let my_user = user();
        let mut recv = Arc::new(Mutex::new(vec![]));
        let mut send = Arc::new(Mutex::new(vec![]));
        if my_user.is_none() {
            state.change_state(GameState::CreateUserB);
            return;
        }
        else {
            if my_user.clone().unwrap().tag == 0 {
                old_manager.for_each_mut(|mut man| {
                    recv = man.grab_in();
                    send = man.grab_out();
                });
            }
            let recv_clone = recv.clone();
            let send_clone = send.clone();
            std::thread::spawn(move || {
                startup(recv_clone, send_clone);
            });
        }
        let entity_ids = vec![
            commands.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    "\u{f790}",
                    TextStyle {
                        font: handles.get_font("KreativeSquare.ttf"),
                        font_size: 34.0,
                        color: Color::BLACK
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Bottom,
                        horizontal: HorizontalAlign::Right
                    }
                ),
                transform: Transform::from_xyz(0.0, 0.0, CURSOR),
                ..Default::default()
            }).insert(CursorMarker {}).id(),
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

        
        if my_user.clone().unwrap().tag != 0 {
            let send_cln = send.clone();
            let mut send_access = send_cln.lock().unwrap();
            send_access.push(Packet::RequestProfile(my_user.unwrap()));
        }
        commands.spawn().insert(
            NewManager::new(entity_ids, recv, send)
        );
    }
}

pub fn new_ui(
    mut commands: Commands,
    mut tb: ResMut<TextBox>,
    manager: Query<&mut NewManager>,
    tb_q: Query<&mut Text, With<crate::components::TextBox>>,
    state: Res<GameState>,
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
                            let out = state_man.grab_out();
                            let mut mode = tb.grab_buffer();
                            mode = String::from(mode.trim_end());
                            mode = String::from(mode.trim_end_matches("\n"));
                            let mut out = out.lock().unwrap();
                            out.push(Packet::CreateWorld(mode.clone()));
                            drop(out);
                            state_man.net_mode();
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
            commands.spawn().insert(PlayManager::new(manager.grab_world()));
            state.change_state(GameState::Play);
        }
    });
}

pub fn new_network(
    query_manager: Query<&mut NewManager>,
) {
    query_manager.for_each_mut(|mut manager| {
        manager.network_step();
    });
}
