use crate::prelude::*;
use bevy_kira_audio::Audio;

pub fn title_screen_loop(
    audio: Res<Audio>,
    core: Res<CoreAssets>,
    audio_serve: Res<Assets<AudioSamples>>
) {
    let samples = audio_serve.get(&core.audio).unwrap();
    audio.play_looped(samples.get("title screen loop"));
}
