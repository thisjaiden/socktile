use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::resources::Disk;

pub fn audio_setup(
    audio: Res<Audio>,
    disk: Res<Disk>
) {
    audio.set_volume(disk.audio_config().volume)
}