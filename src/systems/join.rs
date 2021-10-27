use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::{client::core::startup, components::{CreateUserManager, CursorMarker, JoinManager}, layers::CURSOR, resources::{AssetHandles, GameState}, shared::{netty::Packet, saves::user}};

pub fn join(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<GameState>,
    mut handles: ResMut<AssetHandles>,
    old_manager: Query<&mut CreateUserManager>,
) {
    if state.eq(&GameState::Join) && state.is_changed() {
        let my_user = user();
        let mut recv = Arc::new(Mutex::new(vec![]));
        let mut send = Arc::new(Mutex::new(vec![]));
        if my_user.is_none() {
            state.change_state(GameState::CreateUser);
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
        }).insert(CursorMarker {}).id()];

        
        if my_user.clone().unwrap().tag != 0 {
            let send_cln = send.clone();
            let mut send_access = send_cln.lock().unwrap();
            send_access.push(Packet::RequestProfile(my_user.unwrap()));
        }
        commands.spawn().insert(
            JoinManager::new(entity_ids, recv, send)
        );
    }
}

pub fn join_ui(

) {

}

pub fn join_network(
    mut commands: Commands,
    state: Res<GameState>,
    query_manager: Query<&mut JoinManager>,
) {
    query_manager.for_each_mut(|mut manager| {
        manager.network_step();
    });
}
