use bevy::prelude::*;
use crate::{components::{AnimatorObject, GamePosition}};

use super::AssetHandles;

pub struct Animator {
    /// A list of all animations.
    /// (Animation Type, Frame, ID, Should Loop)
    animations: Vec<(Animation, AnimationFrame, AnimatorID, Loops)>,
    /// The highest unique animation ID.
    top_id: AnimatorID,
    /// A list of animations to play after an animation ends.
    /// (ID of Animation to Play After, (Animation Type, ID, Should Loop))
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
                    self.animations[index].1 = 1;
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
    pub fn step(
        &mut self,
        mut commands: Commands,
        mut handles: ResMut<AssetHandles>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        aos: Query<
            (
                Entity,
                &mut AnimatorObject,
                &mut Transform,
                Option<&mut Handle<ColorMaterial>>,
                Option<&mut Text>
            ),
        >
    ) {
        let anims_cl = self.animations.clone();
        // handle old animations
        let mut anim_index = 0;
        let mut removal_ids = vec![];
        for animation in &anims_cl {
            self.animations[anim_index].1 += 1;
            if animation.0.clone().is_done(animation.1) {
                removal_ids.push(animation.2);
            }
            anim_index += 1;
        }
        // The only reason this is done after is so the iterator doesn't skip animations as the list is shifted
        for id in removal_ids {
            self.request_animation_end_soft(id);
            aos.for_each_mut(
                |(
                    e,
                    object,
                    _transform,
                    _texture,
                    _text
                )| {
                    if object.animation_id == id {
                        for animation in &anims_cl {
                            if animation.2 == id {
                                if !animation.3 {
                                    commands.entity(e).despawn();
                                }
                            }
                        }
                    }
                }
            );
        }
        // spawn new animations
        for animation in &anims_cl {
            // spawn new animations
            if animation.1 == 0 {
                let anim_details = animation.0.clone().details(0);
                for (modal, offset, index) in anim_details.display_modals {
                    match modal {
                        DisplayModal::Sprite(texture_name, layer) => {
                            commands.spawn_bundle(SpriteBundle {
                                transform: Transform::from_xyz(
                                    anim_details.location.x as f32 + offset.x as f32,
                                    anim_details.location.y as f32 + offset.y as f32,
                                    layer
                                ),
                                material: materials.add(handles.get_texture(&texture_name).into()),
                                ..Default::default()
                            }).insert(
                                AnimatorObject {
                                    animation_id: animation.2,
                                    index: index
                                }
                            );
                        }
                        DisplayModal::Text(font, size, color, text) => {
                            commands.spawn_bundle(Text2dBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection {
                                            value: text,
                                            style: TextStyle {
                                                font: handles.get_font(&font),
                                                font_size: size,
                                                color: color
                                            }
                                        }
                                    ],
                                    alignment: TextAlignment {
                                        vertical: VerticalAlign::Top,
                                        horizontal: HorizontalAlign::Right
                                    }
                                },
                                ..Default::default()
                            }).insert(
                                AnimatorObject {
                                    animation_id: animation.2,
                                    index: index
                                }
                            );
                        }
                        DisplayModal::NoUpdate => {
                            unimplemented!();
                        }
                    }
                }
            }
            else { // edit existing animations
                aos.for_each_mut(
                    |(
                        _e,
                        object,
                        mut transform,
                        _texture,
                        mut text
                    )| {
                        if object.animation_id == animation.2 {
                            let anim_details = animation.0.clone().details(animation.1);
                            for (modal, offset, index) in anim_details.display_modals {
                                if index == object.index {
                                    transform.translation.x = anim_details.location.x as f32 + offset.x as f32;
                                    transform.translation.y = anim_details.location.y as f32 + offset.x as f32;
                                    match modal {
                                        DisplayModal::Sprite(_tex_from, _layer) => {
                                            todo!();
                                        },
                                        DisplayModal::Text(font, size, color, text_from) => {
                                            if let Some(ref mut text) = text {
                                                text.sections[0].value = text_from;
                                                text.sections[0].style.color = color;
                                                text.sections[0].style.font_size = size;
                                                text.sections[0].style.font = handles.get_font(&font);
                                            }
                                            else {
                                                panic!("No Text object for Entity that should have had it.");
                                            }
                                        },
                                        DisplayModal::NoUpdate => {
                                            // do nothing
                                        }
                                    }
                                }
                            }
                        }
                    }
                );
            }
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
    TitleScreenBob
}

mod fits;
mod fitsnwifi;
mod fitsnggs;
mod tsb;

impl Animation {
    fn is_done(self, frame: AnimationFrame) -> bool {
        match self {
            Self::FloatInTitleScreen => frame > 10,
            Self::FloatInTitleScreenNoWIFI => frame > 1,
            Self::FloatInTitleScreenNoGGS => frame > 1,
            Self::TitleScreenBob => frame > 40
        }
    }
    fn details(&mut self, frame: AnimationFrame) -> FrameDetails {
        match self {
            Self::FloatInTitleScreen => fits::fits(frame),
            Self::FloatInTitleScreenNoWIFI => fitsnwifi::fitsnwifi(frame),
            Self::FloatInTitleScreenNoGGS => fitsnggs::fitsnggs(frame),
            Self::TitleScreenBob => tsb::tsb(frame),
        }
    }
}

pub struct FrameDetails {
    location: GamePosition,
    display_modals: Vec<(DisplayModal, GamePosition, ObjectIndex)>
}

pub enum DisplayModal {
    Sprite(TextureOrigin, Layer),
    Text(FontOrigin, FontSize, FontColor, String),
    NoUpdate
}

pub type AnimationFrame = usize;
type TextureOrigin = String;
type FontOrigin = String;
type FontSize = f32;
type FontColor = bevy::prelude::Color;
type Layer = f32;
type ObjectIndex = usize;
