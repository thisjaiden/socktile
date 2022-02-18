use bevy::prelude::*;

use crate::{resources::Disk, components::TitleScreenUser};

pub fn update_title_screen_user(
    disk: Res<Disk>,
    mut text: Query<&mut Text, With<TitleScreenUser>>
) {
    if disk.is_changed() {
        let mut txt = text.single_mut();
        txt.sections[0].value = format!(
            "{}#{}",
            disk.user().unwrap().username,
            disk.user().unwrap().tag
        );
    }
}
