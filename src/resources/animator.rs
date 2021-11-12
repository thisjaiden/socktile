use bevy::prelude::*;
use crate::components::{AnimatorObject, GamePosition};

use super::AssetHandles;

pub struct Animator {
    animations: Vec<(Animation, AnimationFrame, AnimatorID, Loops)>,
    top_id: AnimatorID,
    followups: Vec<(AnimatorID, (Animation, AnimatorID, Loops))>
}

impl Animator {
    pub fn init() -> Animator {
        Animator {
            animations: vec![],
            top_id: 0,
            followups: vec![]
        }
    }
    pub fn request_animation(&mut self, animation: Animation, loops: bool) -> AnimatorID {
        self.animations.push((animation, 0, self.top_id, loops));
        self.top_id += 1;
        self.top_id - 1
    }
    pub fn request_animation_end(&mut self, id: AnimatorID) {
        let mut index = 0;
        for animation in self.animations.clone() {
            if animation.2 == id {
                self.animations.swap_remove(index);
                break;
            }
            index += 1;
        }
        for followup in self.followups.clone() {
            if followup.0 == id {
                self.animations.push((followup.1.0, 0, followup.1.1, followup.1.2));
            }
        }
    }
    fn request_animation_end_soft(&mut self, id: AnimatorID) {
        let mut index = 0;
        for animation in self.animations.clone() {
            if animation.2 == id {
                if animation.3 {
                    self.animations[index].1 = 0;
                }
                else {
                    self.animations.swap_remove(index);
                }
                break;
            }
            index += 1;
        }
        for followup in self.followups.clone() {
            if followup.0 == id {
                self.animations.push((followup.1.0, 0, followup.1.1, followup.1.2));
            }
        }
    }
    pub fn request_animation_followup(&mut self, id: AnimatorID, animation: Animation, loops: bool) -> AnimatorID {
        self.followups.push((id, (animation, self.top_id, loops)));
        self.top_id += 1;
        self.top_id - 1
    }
    pub fn animation_exists(&mut self, id: AnimatorID) -> bool {
        for animation in &self.animations {
            if animation.2 == id {
                return true;
            }
        }
        false
    }
    pub fn animation_frame(&mut self, id: AnimatorID) -> AnimationFrame {
        for animation in &self.animations {
            if animation.2 == id {
                return animation.1;
            }
        }
        panic!("Invalid AnimatorID {}!", id);
    }
    pub fn animation_details(&mut self, id: AnimatorID) -> FrameDetails {
        for animation in &self.animations {
            if animation.2 == id {
                return animation.0.clone().details(animation.1);
            }
        }
        panic!("Invalid AnimatorID {}!", id);
    }
    pub fn step(
        &mut self,
        mut commands: Commands,
        mut handles: ResMut<AssetHandles>,
        aos: Query<
            (
                &mut AnimatorObject,
                &mut Transform,
                Option<&mut Handle<ColorMaterial>>,
                Option<&mut Text>
            ),
        >
    ) {
        let anims_cl = self.animations.clone();
        // spawn new animations
        for animation in &anims_cl {

        }
        // edit existing animations

        // handle old animations
        let mut anim_index = 0;
        let mut removal_ids = vec![];
        for animation in anims_cl {
            self.animations[anim_index].1 += 1;
            if animation.0.is_done(animation.1) {
                removal_ids.push(animation.2);
            }
            anim_index += 1;
        }
        // The only reason this is done after is so the iterator doesn't skip animations as the list is shifted
        for id in removal_ids {
            self.request_animation_end_soft(id);
        }
    }
}

pub type AnimatorID = usize;
pub type Loops = bool;

#[derive(Debug, Clone)]
pub enum Animation {
    FloatInTitleScreen,
    FloatInTitleScreenNoWIFI,
    FloatInTitleScreenNoGGS,
}

impl Animation {
    fn is_done(self, frame: AnimationFrame) -> bool {
        match self {
            Self::FloatInTitleScreen => frame > 10,
            Self::FloatInTitleScreenNoWIFI => frame > 10,
            Self::FloatInTitleScreenNoGGS => frame > 10
        }
    }
    fn details(&mut self, frame: AnimationFrame) -> FrameDetails {
        match self {
            _ => {
                panic!("No animation data for {:?} frame {}!", self, frame);
            }
        }
    }
}

pub type AnimationFrame = usize;


pub struct FrameDetails {
    location: GamePosition,
    display_modal: DisplayModal
}

pub enum DisplayModal {
    Sprite(TextureOrigin, Flipped),
    Text(FontOrigin, FontSize, FontColor),
    Multi(Vec<DisplayModal>)
}

type TextureOrigin = String;
type FontOrigin = String;
type FontSize = f32;
type FontColor = bevy::prelude::Color;
type Flipped = bool;
