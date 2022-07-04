use bevy::prelude::*;
use crate::{
    assets::{FontAssets, CoreAssets},
    components::ldtk::TileMarker,
    components::TitleScreenUser,
    shared::{netty::Packet},
    resources::{Netty, Disk},
    GameState,
    consts::UI_TEXT
};

pub fn load_title_screen_map(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    font_assets: Res<FontAssets>,
    mut netty: ResMut<Netty>,
    disk: Res<Disk>,
    core: Res<CoreAssets>
) {
    if disk.user().is_some() {
        commands.spawn_bundle(SpriteBundle {
            texture: core.title_screen.clone(),
            ..default()
        })
        .insert(TileMarker {});

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
        .insert(TileMarker {});
        netty.say(Packet::UserPresence(disk.user().unwrap()));
    }
    else {
        state.set(GameState::MakeUser).unwrap();
    }
}
