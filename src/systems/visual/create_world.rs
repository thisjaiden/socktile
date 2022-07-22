use bevy::prelude::*;

use crate::{assets::{FontAssets, CoreAssets}, modular_assets::ModularAssets, resources::ui::{UIManager, UIClickable, UIClickAction}, consts::{BACKGROUND, UI_TEXT}, components::RemoveOnStateChange};

pub fn create_world(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    core: Res<CoreAssets>,
    core_serve: Res<Assets<ModularAssets>>,
    mut ui: ResMut<UIManager>
) {
    let core_assets = core_serve.get(core.core.clone()).unwrap();
    // Background
    commands.spawn_bundle(SpriteBundle {
        texture: core.create_world.clone(),
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND),
        ..default()
    })
    .insert(RemoveOnStateChange {});
    // Cancel text
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: core_assets.get_lang("en_us.core.create_world.cancel"),
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
            -500.0,
            -300.0,
            UI_TEXT
        ),
        ..default()
    })
    .insert(RemoveOnStateChange {});
    ui.add_ui(UIClickable {
        action: UIClickAction::GoToTitleScreen,
        location: (-600.0, -250.0),
        size: (200.0, 100.0),
        ..default()
    });
    // Confirm text
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: core_assets.get_lang("en_us.core.create_world.confirm"),
                    style: TextStyle {
                        font: font_assets.apple_tea.clone(),
                        font_size: 64.0,
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
            100.0,
            -300.0,
            UI_TEXT
        ),
        ..default()
    })
    .insert(RemoveOnStateChange {});
    ui.add_ui(UIClickable {
        action: UIClickAction::CreateWorld,
        location: (100.0, -250.0),
        size: (500.0, 100.0),
        removed_on_use: false,
        ..default()
    });
    // Description text
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: core_assets.get_lang("en_us.core.create_world.description"),
                    style: TextStyle {
                        font: font_assets.apple_tea.clone(),
                        font_size: 54.0,
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
            0.0,
            300.0,
            UI_TEXT
        ),
        ..default()
    })
    .insert(RemoveOnStateChange {});
}