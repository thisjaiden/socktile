use crate::prelude::*;
use bevy_kira_audio::Audio;

pub fn title_screen_loop(
    audio: Res<Audio>,
    assets: Res<CoreAssets>,
    server: Res<Assets<ModularAssets>>
) {
    let core_assets = server.get(&assets.core).unwrap();
    audio.play_looped(core_assets.get_audio("title screen loop"));
}
