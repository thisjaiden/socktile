use bevy::prelude::*;
use crate::assets::FontAssets;
use crate::components::UILocked;
use crate::{components::CursorMarker};
use crate::consts::CURSOR;

pub fn cursor(
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<CursorMarker>>
) {
    for mut transform in query.iter_mut() {
        let p_window = windows.get_primary().unwrap();
        let cursor_pos = p_window.cursor_position();
        if let Some(position) = cursor_pos {
            transform.translation.x = (position.x * 2.0) - (p_window.width() / 2.0) - 7.0;
            transform.translation.y = (position.y * 2.0) - (p_window.height() / 2.0) + 5.0;
            transform.translation.z = CURSOR;
        }
    }
}


pub fn spawn(
    mut commands: Commands,
    fonts: Res<FontAssets>
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            '\u{f790}',
            TextStyle {
                font: fonts.kreative_square.clone(),
                font_size: 34.0,
                color: Color::BLACK
            },
            TextAlignment {
                vertical: VerticalAlign::Bottom,
                horizontal: HorizontalAlign::Right
            }
        ),
        transform: Transform::from_xyz(0.0, 0.0, CURSOR),
        ..Default::default()
    }).insert(CursorMarker {}).insert(UILocked {});
}
