use bevy::prelude::*;
use bevy_kira_audio::Audio;
use crate::{assets::CoreAssets, modular_assets::ModularAssets};

pub fn title_screen_loop(
    audio: Res<Audio>,
    assets: Res<CoreAssets>,
    server: Res<Assets<ModularAssets>>
) {
    let core_assets = server.get(assets.core.clone()).unwrap();
    audio.play_looped(core_assets.get_audio("title screen loop"));
}
