use crate::components::GamePosition;

pub struct Animator {
    animations: Vec<(Animation, AnimationFrame, AnimatorID, Loops)>,
    top_id: AnimatorID,
    followups: Vec<(AnimatorID, (Animation, AnimatorID))>
}

impl Animator {
    pub fn init() -> Animator {
        Animator {
            animations: vec![],
            top_id: 0,
            followups: vec![]
        }
    }
    pub fn request_animation(&mut self, animation: Animation) -> AnimatorID {
        self.animations.push((animation, 0, self.top_id, false));
        self.top_id += 1;
        self.top_id - 1
    }
    pub fn request_looping_animation(&mut self, animation: Animation) -> AnimatorID {
        self.animations.push((animation, 0, self.top_id, true));
        self.top_id += 1;
        self.top_id - 1
    }
    pub fn request_animation_end(&mut self, id: AnimatorID) {
        
    }
    pub fn request_animation_followup(&mut self, id: AnimatorID, animation: Animation) -> AnimatorID {
        self.followups.push((id, (animation, self.top_id)));
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
    pub fn step(&mut self) {

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
    fn is_done(&mut self, frame: AnimationFrame) -> bool {
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
    Text(FontOrigin, FontSize, FontColor)
}

type TextureOrigin = String;
pub type FontOrigin = String;
pub type FontSize = f32;
pub type FontColor = bevy::prelude::Color;
pub type Flipped = bool;
