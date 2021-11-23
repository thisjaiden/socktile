use bevy::prelude::*;

use crate::{resources::{AssetHandles, GameState, Netty}, shared::{netty::Packet, saves::user}};

pub fn join(
    mut state: ResMut<GameState>,
    mut netty: ResMut<Netty>
) {
    if state.eq(&GameState::Join) && state.is_changed() {
        let my_user = user();
        if my_user.is_none() {
            state.change_state(GameState::CreateUser);
            return;
        }

        
        if my_user.clone().unwrap().tag != 0 {
            netty.say(Packet::RequestProfile(my_user.unwrap()));
        }
    }
}

pub fn join_ui_create(
    mut _commands: Commands,
    mut _handles: ResMut<AssetHandles>,
    state: Res<GameState>,
) {
    if state.eq(&GameState::Join) {

    }
}

pub fn join_ui_update(

) {

}
