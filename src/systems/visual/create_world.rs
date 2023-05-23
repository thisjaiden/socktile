use crate::prelude::*;

pub fn create_world(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    core: Res<CoreAssets>,
    lang_serve: Res<Assets<LanguageKeys>>,
    mut ui: ResMut<UIManager>,
) {
    let lang = lang_serve.get(&core.lang).unwrap();
    // Background
    commands.spawn((
        SpriteBundle {
            texture: core.create_world.clone(),
            transform: Transform::from_xyz(0.0, 0.0, BACKGROUND),
            ..default()
        },
        RemoveOnStateChange {},
    ));
    // Cancel text
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: lang.get("en_us.core.create_world.cancel"),
                    style: TextStyle {
                        font: font_assets.apple_tea.clone(),
                        font_size: 64.0,
                        color: Color::BLACK,
                    },
                }],
                alignment: TextAlignment::Center,
                linebreak_behaviour: bevy::text::BreakLineOn::AnyCharacter
            },
            transform: Transform::from_xyz(-500.0, -300.0, UI_TEXT),
            ..default()
        },
        RemoveOnStateChange {},
    ));
    ui.add_ui(UIClickable {
        action: UIClickAction::GoToTitleScreen,
        location: (-600.0, -250.0),
        size: (200.0, 100.0),
        ..default()
    });
    // Confirm text
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: lang.get("en_us.core.create_world.confirm"),
                    style: TextStyle {
                        font: font_assets.apple_tea.clone(),
                        font_size: 64.0,
                        color: Color::BLACK,
                    },
                }],
                alignment: TextAlignment::Left,
                linebreak_behaviour: bevy::text::BreakLineOn::AnyCharacter
            },
            transform: Transform::from_xyz(100.0, -300.0, UI_TEXT),
            ..default()
        },
        RemoveOnStateChange {},
    ));
    ui.add_ui(UIClickable {
        action: UIClickAction::CreateWorld,
        location: (100.0, -250.0),
        size: (500.0, 100.0),
        removed_on_use: false,
        ..default()
    });
    // Description text
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: lang.get("en_us.core.create_world.description"),
                    style: TextStyle {
                        font: font_assets.apple_tea.clone(),
                        font_size: 54.0,
                        color: Color::BLACK,
                    },
                }],
                alignment: TextAlignment::Center,
                linebreak_behaviour: bevy::text::BreakLineOn::AnyCharacter
            },
            transform: Transform::from_xyz(0.0, 300.0, UI_TEXT),
            ..default()
        },
        RemoveOnStateChange {},
    ));
}
