use crate::prelude::*;
use bevy_kira_audio::Audio;

pub fn audio_setup(
    audio: Res<Audio>,
    disk: Res<Disk>
) {
    audio.set_volume(disk.audio_config().volume)
}