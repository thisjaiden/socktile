use bevy::prelude::*;

use crate::{DEV_BUILD, components::SettingsManager, layers::UI_TEXT, resources::{AssetHandles, GameState}};

pub fn settings(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    state: Res<GameState>,
    mut handles: ResMut<AssetHandles>
) {
    if state.eq(&GameState::Settings) && state.is_changed() {
        let mut entity_ids = vec![];
        let system_text = format!("{}-{}-{}", std::env::consts::ARCH, std::env::consts::FAMILY, std::env::consts::OS);
        if DEV_BUILD {
            entity_ids.push(commands.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    system_text,
                    TextStyle {
                        font: handles.get_font("base.ttf"),
                        font_size: 42.0,
                        color: Color::BLACK
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center
                    }
                ),
                transform: Transform::from_xyz(-1920.0 /2. + 32.0, -1080.0 / 2.0 + 32.0, UI_TEXT),
                ..Default::default()
            }).id());
            commands.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    "system_text",
                    TextStyle {
                        font: handles.get_font("base.ttf"),
                        font_size: 34.0,
                        color: Color::BLACK
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center
                    }
                ),
                transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
                ..Default::default()
            });
        }
        entity_ids.push(commands.spawn_bundle(SpriteBundle {
            material: materials.add(handles.get_texture("gblin_exp_2.png").into()),
            ..Default::default()
        }).id());
        entity_ids.push(commands.spawn_bundle(SpriteBundle {
            material: materials.add(handles.get_texture("ts.png").into()),
            ..Default::default()
        }).id());
        commands.spawn().insert(SettingsManager::new(entity_ids));
    }
    if state.eq(&GameState::Settings) {
        
    }
}
