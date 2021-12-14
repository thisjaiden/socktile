use bevy::prelude::*;

use crate::{resources::{Netty, ConnectionStatus, Reality}, GameState};

pub fn startup_checks(
    mut netty: ResMut<Netty>,
    mut state: ResMut<State<GameState>>
) {
    match netty.connection() {
        ConnectionStatus::Connected | ConnectionStatus::Stable => {
            state.set(GameState::TitleScreen).unwrap();
        }
        _ => {
            state.set(GameState::OfflineTitle).unwrap();
        }
    }
}

pub fn step(
    mut netty: ResMut<Netty>,
    mut reality: ResMut<Reality>
) {
    netty.step(&mut reality);
}
