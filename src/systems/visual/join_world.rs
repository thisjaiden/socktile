use crate::prelude::*;

pub fn join_world(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    core: Res<CoreAssets>,
    lang_serve: Res<Assets<LanguageKeys>>,
    mut ui: ResMut<UIManager>
) {
    let lang = lang_serve.get(&core.lang).unwrap();
    // Background
    commands.spawn((
        SpriteBundle {
            texture: core.join_world.clone(),
            transform: Transform::from_xyz(0.0, 0.0, BACKGROUND),
            ..default()
        },
        RemoveOnStateChange {}
    ));
    // Cancel text
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: lang.get("en_us.core.join_world.cancel"),
                        style: TextStyle {
                            font: font_assets.apple_tea.clone(),
                            font_size: 64.0,
                            color: Color::BLACK
                        }
                    }
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                }
            },
            transform: Transform::from_xyz(
                -600.0,
                -400.0,
                UI_TEXT
            ),
            ..default()
        },
        RemoveOnStateChange {}
    ));
    ui.add_ui(UIClickable {
        action: UIClickAction::GoToTitleScreen,
        location: (-700.0, -350.0),
        size: (200.0, 100.0),
        ..default()
    });
}
