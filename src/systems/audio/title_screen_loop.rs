use bevy::prelude::*;
use bevy_kira_audio::Audio;
use crate::assets::AudioAssets;

pub fn title_screen_loop(
    audio: Res<Audio>,
    samples: Res<AudioAssets>
) {
    audio.play_looped(samples.title_screen_loop.clone());
}
