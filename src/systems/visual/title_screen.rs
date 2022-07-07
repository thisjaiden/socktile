use bevy::prelude::*;
use crate::{
    assets::{FontAssets, CoreAssets},
    components::RemoveOnStateChange,
    components::TitleScreenUser,
    consts::UI_TEXT
};

pub fn title_screen(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    core: Res<CoreAssets>
) {
    commands.spawn_bundle(SpriteBundle {
        texture: core.title_screen.clone(),
        ..default()
    })
    .insert(RemoveOnStateChange {});

    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: String::new(),
                    style: TextStyle {
                        font: font_assets.simvoni_bold.clone(),
                        font_size: 44.0,
                        color: Color::BLACK
                    }
                }
            ],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left
            }
        },
        transform: Transform::from_xyz(
            -(1920.0 / 2.0) + 8.0,
            -(1080.0 / 2.0) + 32.0,
            UI_TEXT
        ),
        ..default()
    })
    .insert(TitleScreenUser {})
    .insert(RemoveOnStateChange {});
}
