pub use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::{components::{ldtk::PlayerMarker}, assets::AnimatorAssets};

pub struct Animator {
    player_prev_pos: HashMap<PlayerMarker, Transform>,
    last_dir_left: HashMap<PlayerMarker, bool>
}

impl Animator {
    pub fn init() -> Animator {
        Animator {
            player_prev_pos: HashMap::default(),
            last_dir_left: HashMap::default()
        }
    }
    pub fn system_player_animator(
        mut selfs: ResMut<Animator>,
        materials: Res<AnimatorAssets>,
        mut players: Query<(&mut Handle<Image>, &mut Transform, &mut PlayerMarker)>
    ) {
        players.for_each_mut(|(mut tex, mut loc, mark)| {
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
            let mut left = false;
            let mut right = false;
            if selfs.player_prev_pos.contains_key(&mark) {
                let trans = selfs.player_prev_pos.get(&mark).unwrap().translation;
                if loc.translation.x > trans.x {
                    right = true;
                }
                if loc.translation.x < trans.x {
                    left = true;
                }
                tex.set(Box::new(materials.forward_idle.clone())).unwrap();
                if right && !left {
                    selfs.last_dir_left.insert(mark.clone(), false);
                }
                else if left {
                    selfs.last_dir_left.insert(mark.clone(), true);
                }
            }
            selfs.player_prev_pos.insert(mark.clone(), *loc);
        });
    }
}
