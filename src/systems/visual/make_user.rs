use bevy::prelude::*;
use crate::{assets::{FontAssets, CoreAssets}, components::{TextBox, RemoveOnStateChange}, consts::{BACKGROUND, UI_TEXT}};

pub fn make_user(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    core: Res<CoreAssets>
) {
    commands.spawn_bundle(SpriteBundle {
        texture: core.create_user.clone(),
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND),
        ..default()
    })
    .insert(RemoveOnStateChange {});

    commands.spawn_bundle(Text2dBundle {
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
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center
            }
        },
        transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
        ..Default::default()
    })
    .insert(TextBox {})
    .insert(RemoveOnStateChange {});
}
