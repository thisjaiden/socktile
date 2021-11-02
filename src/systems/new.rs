use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::{client::core::startup, components::CreateUserManager, resources::{AssetHandles, GameState}, shared::saves::user};

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
    }
}
