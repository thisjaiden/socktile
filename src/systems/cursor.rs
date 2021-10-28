use bevy::prelude::*;
use crate::{components::CursorMarker, resources::GameState};
use crate::layers::CURSOR;

pub fn cursor(
    windows: Res<Windows>,
    state: Res<GameState>,
    mut cursors: Query<&mut Transform, With<CursorMarker>>
) {
    for mut transform in cursors.iter_mut() {
        if state.eq(&GameState::TitleScreen) || state.eq(&GameState::Settings) {
            let p_window = windows.get_primary().unwrap();
            let cursor_pos = p_window.cursor_position();
            if let Some(position) = cursor_pos {
                transform.translation.x = position.x - (p_window.width() * p_window.scale_factor() as f32 / 2.0) - 7.0;
                transform.translation.y = position.y - (p_window.height() * p_window.scale_factor() as f32 / 2.0) + 5.0;
                transform.translation.z = CURSOR;
            }
        }
    }
}
