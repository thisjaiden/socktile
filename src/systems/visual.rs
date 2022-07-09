mod title_screen;
pub use title_screen::title_screen;
mod make_user;
pub use make_user::make_user;
mod update_title_screen_user;
pub use update_title_screen_user::*;

use bevy::prelude::*;

use crate::components::RemoveOnStateChange;

pub fn clear_old(
    mut commands: Commands,
    query: Query<Entity, With<RemoveOnStateChange>>
) {
    query.for_each(|e| {
        commands.entity(e).despawn();
    });
}
