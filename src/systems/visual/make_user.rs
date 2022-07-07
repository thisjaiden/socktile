use bevy::prelude::*;
use crate::{assets::{FontAssets, CoreAssets}, components::{TextBox, RemoveOnStateChange}};

pub fn make_user(
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
        ..Default::default()
    })
    .insert(TextBox {})
    .insert(RemoveOnStateChange {});
}
