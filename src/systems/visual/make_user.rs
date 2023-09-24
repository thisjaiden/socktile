use crate::prelude::*;

pub fn make_user(mut commands: Commands, font_assets: Res<FontAssets>, core: Res<CoreAssets>) {
    info!("Spawning user creation entities!");
    commands.spawn((
        SpriteBundle {
            texture: core.create_user.clone(),
            transform: Transform::from_xyz(0.0, 0.0, BACKGROUND),
            ..default()
        },
        RemoveOnStateChange {},
    ));

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: String::new(),
                        style: TextStyle {
                            font: font_assets.simvoni.clone(),
                            font_size: 35.0,
                            color: Color::BLACK
                        }
                    }
                    ;2 // we make two text components to show errors below if applicable
                ],
                alignment: TextAlignment::Center,
                linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter
            },
            transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
            ..Default::default()
        },
        TextBox {},
        RemoveOnStateChange {},
    ));
}
