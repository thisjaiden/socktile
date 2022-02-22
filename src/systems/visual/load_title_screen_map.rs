use bevy::prelude::*;
use crate::{assets::{MapAssets, FontAssets}, ldtk::{LDtkMap, load_level}, components::ldtk::TileMarker, components::TitleScreenUser, shared::{netty::Packet}, GameState, resources::{ui::UIManager, Netty, Disk}, consts::UI_TEXT};

pub fn load_title_screen_map(
    mut commands: Commands,
    unloads: Query<Entity, With<TileMarker>>,
    mut maps: ResMut<Assets<LDtkMap>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    target_maps: Res<MapAssets>,
    mut state: ResMut<State<GameState>>,
    font_assets: Res<FontAssets>,
    uiman: ResMut<UIManager>,
    mut netty: ResMut<Netty>,
    disk: Res<Disk>
) {
    if disk.user().is_some() {
        let a = maps.get_mut(target_maps.core.clone()).unwrap();
        let level = a.get_level("Title_screen");
        load_level(unloads, level, a, texture_atlases, font_assets.clone(), uiman, &mut commands);
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
                -(1920.0 / 2.0),
                -(1080.0 / 2.0) + 32.0,
                UI_TEXT
            ),
            ..Default::default()
        })
        .insert(TitleScreenUser {})
        .insert(TileMarker {});
        netty.say(Packet::UserPresence(disk.user().unwrap()));
    }
    else {
        state.set(GameState::MakeUser).unwrap();
    }
}
