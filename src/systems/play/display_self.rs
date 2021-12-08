use bevy::prelude::*;

use crate::{resources::{Reality, GameState}, layers::PLAYER_CHARACTERS};

pub fn on_start(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut reality: ResMut<Reality>
) {
    if game_state.eq(&GameState::Play) && game_state.is_changed() {
        reality.register_player(
            commands.spawn_bundle(SpriteBundle {
                ..Default::default()
            }).id()
        );
    }
}

pub fn on_tick(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut reality: ResMut<Reality>
) {
    
    commands.entity(reality.player_id()).insert(Transform::from_xyz(4.0, 50.0, PLAYER_CHARACTERS));
}