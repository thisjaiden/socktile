pub use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::{components::{ldtk::PlayerMarker}, assets::AnimatorAssets};

pub struct Animator {
    player_prev_pos: HashMap<PlayerMarker, Transform>,
    last_dir_left: HashMap<PlayerMarker, bool>,
    /// =0 | not idling
    /// >0 | num of frames idling
    /// resets to 1 every 100 frames (101 -> 1)
    idle_animation_state: HashMap<PlayerMarker, u8>
}

impl Animator {
    pub fn init() -> Animator {
        Animator {
            player_prev_pos: HashMap::default(),
            last_dir_left: HashMap::default(),
            idle_animation_state: HashMap::default()
        }
    }
    pub fn system_player_initiator(
        mut selfs: ResMut<Animator>
    ) {
        if selfs.idle_animation_state.len() < selfs.player_prev_pos.len() {
            for (key, _value) in selfs.player_prev_pos.clone() {
                if !selfs.idle_animation_state.contains_key(&key) {
                    selfs.idle_animation_state.insert(key, 0);
                }
            }
        }
    }
    pub fn system_player_animator(
        mut selfs: ResMut<Animator>,
        materials: Res<AnimatorAssets>,
        mut players: Query<(&mut Handle<Image>, &mut Transform, &mut PlayerMarker)>
    ) {
        // Animation for every player
        players.for_each_mut(|(mut tex, mut loc, mark)| {
            // Flip animation
            if selfs.last_dir_left.contains_key(&mark) {
                if *selfs.last_dir_left.get(&mark).unwrap() {
                    if loc.scale.x > -1.0 {
                        loc.scale.x -= 0.1;
                    }
                }
                else {
                    if loc.scale.x < 1.0 {
                        loc.scale.x += 0.1;
                    }
                }
            }
            // Saftey check: If the player has existed for 2 frames
            if selfs.player_prev_pos.contains_key(&mark) {
                // Get a copy of the player's position last frame
                let previous_translation = selfs.player_prev_pos.get(&mark).unwrap().translation;

                // If the player has moved left or right, mark the flip animation to trigger for that direction
                if loc.translation.x > previous_translation.x {
                    selfs.last_dir_left.insert(mark.clone(), false);
                }
                else if loc.translation.x < previous_translation.x {
                    selfs.last_dir_left.insert(mark.clone(), true);
                }
                if loc.translation == previous_translation {
                    // If the player hasn't moved, play the idle animation.

                    // Grab current animation state value.
                    let state = *selfs.idle_animation_state.get(&mark).unwrap();

                    // Determine new animation state value.
                    let new_state;
                    if state > 100 {
                        new_state = 1;
                    }
                    else {
                        new_state = state + 1;
                    }

                    // Update index with the new value.
                    selfs.idle_animation_state.insert(mark.clone(), new_state);

                    // Update sprite, if appropriate.
                    match new_state {
                        1 => tex.set(Box::new(materials.idle0.clone())).unwrap(),
                        51 => tex.set(Box::new(materials.idle1.clone())).unwrap(),
                        _ => {} // Do nothing this frame.
                    }
                }
                else {
                    // The player moved, disable idle animation.
                    selfs.idle_animation_state.insert(mark.clone(), 0);
                }
            }
            selfs.player_prev_pos.insert(mark.clone(), *loc);
        });
    }
}
