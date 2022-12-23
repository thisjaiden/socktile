use crate::prelude::*;

pub fn animate_sprites(
    time: Res<Time>,
    core: Res<CoreAssets>,
    mut anim_sprite_ref: ResMut<Assets<AnimatedSprite>>,
    mut q: Query<(&mut Handle<AnimatedSprite>, &mut Handle<Image>)>
) {
    q.for_each_mut(|(controller_ref, mut sprite)| {
        let pot_controller = anim_sprite_ref.get_mut(&controller_ref);
        if let Some(controller) = pot_controller {
            controller.update(time.clone(), &mut sprite, core.blank.clone());
        }
    })
}
