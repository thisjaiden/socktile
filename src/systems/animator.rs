use bevy::prelude::*;

use crate::{components::AnimatorObject, resources::{Animator, AssetHandles}};

pub struct AnimatorTimer(pub Timer);

pub fn animator(
    mut commands: Commands,
    mut handles: ResMut<AssetHandles>,
    mut animator: ResMut<Animator>,
    time: Res<Time>,
    mut timer: ResMut<AnimatorTimer>,
    aos: Query<
        (
            &mut AnimatorObject,
            &mut Transform,
            Option<&mut Handle<ColorMaterial>>,
            Option<&mut Text>
        )
    >
) {
    if timer.0.tick(time.delta()).just_finished() {
        animator.step(commands, handles, aos);
    }
}
