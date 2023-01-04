use crate::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub fn cursor(
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<CursorMarker>>,
    mut lines: ResMut<DebugLines>,
) {
    for mut transform in query.iter_mut() {
        let p_window = windows.get_primary().unwrap();
        let cursor_pos = p_window.cursor_position();
        if let Some(position) = cursor_pos {
            transform.translation.x = position.x - (p_window.width() / 2.0);
            transform.translation.y = position.y - (p_window.height() / 2.0);
            transform.translation.z = CURSOR;
            if UI_DEBUG {
                let line_pos = Vec3::new(
                    transform.translation.x + CURSOR_OFFSET[0] - 5.0,
                    transform.translation.y + CURSOR_OFFSET[1] - 5.0,
                    DEBUG,
                );
                let mut line_end = line_pos;
                line_end.x += 10.0;
                line_end.y += 10.0;
                let mut line_pos_2 = line_pos;
                line_pos_2.y += 10.0;
                let mut line_end_2 = line_pos;
                line_end_2.x += 10.0;
                lines.line_colored(line_pos, line_end, 0.0, Color::ORANGE);
                lines.line_colored(line_pos_2, line_end_2, 0.0, Color::ORANGE);
            }
        }
    }
}

pub fn spawn(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: String::from('\u{f790}'),
                    style: TextStyle {
                        font: fonts.kreative_square.clone(),
                        font_size: 34.0,
                        color: Color::BLACK,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Right,
                },
            },
            transform: Transform::from_xyz(0.0, 0.0, CURSOR),
            ..Default::default()
        },
        CursorMarker {},
        UILocked {},
    ));
}
