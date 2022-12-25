use crate::prelude::*;

pub fn animate_sprites(
    mut commands: Commands,
    time: Res<Time>,
    core: Res<CoreAssets>,
    mut q: Query<(Entity, &mut AnimatedSprite, &mut Handle<Image>)>
) {
    q.for_each_mut(|(entity, mut controller, mut sprite)| {
        controller.update(time.clone(), &mut sprite, core.blank.clone(), commands.entity(entity));
    })
}
