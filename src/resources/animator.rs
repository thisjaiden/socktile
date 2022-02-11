pub use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::{components::{ldtk::PlayerMarker}, assets::AnimatorAssets};

pub struct Animator {
    player_prev_pos: HashMap<PlayerMarker, Transform>,
}

impl Animator {
    pub fn init() -> Animator {
        Animator {
            player_prev_pos: HashMap::default()
        }
    }
    pub fn system_player_animator(
        mut selfs: ResMut<Animator>,
        materials: Res<AnimatorAssets>,
        mut players: Query<(&mut Handle<Image>, &mut Transform, &mut PlayerMarker)>
    ) {
        players.for_each_mut(|(mut tex, loc, mark)| {
            let mut left = false;
            let mut down = false;
            let mut right = false;
            let mut up = false;
            if selfs.player_prev_pos.contains_key(&mark) {
                let trans = selfs.player_prev_pos.get(&mark).unwrap().translation;
                if loc.translation.x > trans.x {
                    right = true;
                }
                if loc.translation.x < trans.x {
                    left = true;
                }
                if loc.translation.y > trans.y {
                    up = true;
                }
                if loc.translation.y < trans.y {
                    down = true;
                }
                if up && left {
                    tex.set(Box::new(materials.face_up_left.clone())).unwrap();
                }
                else if up && right {
                    tex.set(Box::new(materials.face_up_right.clone())).unwrap();
                }
                else if down && right {
                    tex.set(Box::new(materials.face_down_right.clone())).unwrap();
                }
                else if down && left {
                    tex.set(Box::new(materials.face_down_left.clone())).unwrap();
                }
                else if up {
                    tex.set(Box::new(materials.face_up.clone())).unwrap();
                }
                else if right {
                    tex.set(Box::new(materials.face_right.clone())).unwrap();
                }
                else if left {
                    tex.set(Box::new(materials.face_left.clone())).unwrap();
                }
                else if down {
                    tex.set(Box::new(materials.face_down.clone())).unwrap();
                }
            }
            selfs.player_prev_pos.insert(mark.clone(), *loc);
        });
    }
}
