use bevy::prelude::*;
use bevy_kira_audio::Audio;
use crate::{assets::CoreAssets, modular_assets::ModularAssets};

pub fn title_screen_loop(
    audio: Res<Audio>,
    assets: Res<CoreAssets>,
    server: Assets<ModularAssets>
) {
    let core_assets = server.get(assets.core).unwrap();
    audio.play_looped(core_assets.get_audio(String::from("title screen loop")));
}
