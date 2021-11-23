use bevy::prelude::*;

use crate::resources::GameState;

pub fn settings(
    state: Res<GameState>,
) {
    if state.eq(&GameState::Settings) && state.is_changed() {
        
    }
    if state.eq(&GameState::Settings) {
        
    }
}
