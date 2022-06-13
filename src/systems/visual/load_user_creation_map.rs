use bevy::prelude::*;
use crate::{assets::{MapAssets, FontAssets}, ldtk::{LDtkMap, load_level}, components::{ldtk::TileMarker, TextBox}, resources::ui::UIManager};

pub fn load_user_creation_map(
    mut commands: Commands,
    unloads: Query<Entity, With<TileMarker>>,
    mut maps: ResMut<Assets<LDtkMap>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    target_maps: Res<MapAssets>,
    font_assets: Res<FontAssets>,
    uiman: ResMut<UIManager>
) {
    let a = maps.get_mut(target_maps.core.clone()).unwrap();
    let level = a.get_level("Create_user");
    load_level(unloads, level, a, texture_atlases, font_assets.clone(), uiman, &mut commands);
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
    }).insert(TextBox {});
}
