use bevy::prelude::*;
use crate::{
    assets::{FontAssets, CoreAssets},
    components::RemoveOnStateChange,
    components::TitleScreenUser,
    consts::{UI_TEXT, BACKGROUND}, resources::ui::{UIManager, UIClickable, UIClickAction}
};

pub fn title_screen(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    core: Res<CoreAssets>,
    mut ui: ResMut<UIManager>
) {
    commands.spawn_bundle(SpriteBundle {
        texture: core.title_screen.clone(),
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
    ui.add_ui(UIClickable {
        action: UIClickAction::CloseProgram,
        location: (-50.0, -100.0),
        size: (50.0, 50.0),
        ..default()
    });
    ui.add_ui(UIClickable {
        action: UIClickAction::OpenSettings,
        location: (50.0, -100.0),
        size: (50.0, 50.0),
        ..default()
    });
    ui.add_ui(UIClickable {
        action: UIClickAction::CreateWorld,
        location: (50.0, 100.0),
        size: (50.0, 50.0),
        ..default()
    });
    ui.add_ui(UIClickable {
        action: UIClickAction::ViewWorldList,
        location: (-50.0, 100.0),
        size: (50.0, 50.0),
        ..default()
    });
}
