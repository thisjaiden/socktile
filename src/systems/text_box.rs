use bevy::prelude::*;

use crate::resources::TextBox;

pub fn text_box(
    mut tb: ResMut<TextBox>,
    ra: Res<Input<KeyCode>>
) {
    tb.update_buffer(ra);
}
