use bevy::prelude::*;

use crate::resources::{AssetHandles, GameState, TextBox};

pub fn create_user(
    mut tb: ResMut<TextBox>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<GameState>,
    mut handles: ResMut<AssetHandles>,
) {
    if state.eq(&GameState::CreateUser) && state.is_changed() {
        tb.clear_buffer();
    }
}
