pub use bevy::prelude::*;

use crate::{components::{ldtk::PlayerMarker, GamePosition}, AnimatorAssets};

use super::Reality;

pub struct Animator {
    player_prev_pos: GamePosition,
    player_animate_index: usize
}

impl Animator {
    pub fn init() -> Animator {
        Animator {
            player_prev_pos: GamePosition { x: 0.0, y: 0.0 },
            player_animate_index: 0
        }
    }
    pub fn system_player_animator(
        selfs: ResMut<Reality>,
        materials: Res<AnimatorAssets>,
        mut players: Query<(&mut Handle<Image>, With<PlayerMarker>)>
    ) {
        // TODO!
    }
}
