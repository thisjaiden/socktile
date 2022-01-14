use bevy::prelude::*;
use bevy::render::camera::Camera;
use crate::FontAssets;
use crate::{components::CursorMarker};
use crate::layers::CURSOR;

pub fn cursor(
    windows: Res<Windows>,
    mut qset: QuerySet<(
        QueryState<&mut Transform, With<CursorMarker>>,
        QueryState<&mut Transform, With<Camera>>
    )>
) {

    let mut camx = 0.0;
    let mut camy = 0.0;
    for transform in qset.q1().iter_mut() {
        camx = transform.translation.x;
        camy = transform.translation.y;
    }
    for mut transform in qset.q0().iter_mut() {
        let p_window = windows.get_primary().unwrap();
        let cursor_pos = p_window.cursor_position();
        if let Some(position) = cursor_pos {
            transform.translation.x = (position.x * 2.0) - (p_window.width() / 2.0) - 7.0 + camx;
            transform.translation.y = (position.y * 2.0) - (p_window.height() / 2.0) + 5.0 + camy;
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
    }).insert(CursorMarker {});
}
