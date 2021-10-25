use bevy::prelude::*;
use crate::{components::{CursorMarker, TitleScreenManager}, layers::{CURSOR, UI_TEXT}, resources::{AssetHandles, GameState}};

pub fn title_screen(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    state: Res<GameState>,
    mut handles: ResMut<AssetHandles>,
) {
    if state.eq(&GameState::TitleScreen) && state.is_changed() {
        let mut entity_ids = vec![];
        entity_ids.push(commands.spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Play",
                TextStyle {
                    font: handles.get_font("base.ttf"),
                    font_size: 52.0,
                    color: Color::BLACK
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Right
                }
            ),
            transform: Transform::from_xyz(512.0, 256.0, UI_TEXT),
            ..Default::default()
        }).id());
        entity_ids.push(commands.spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "New",
                TextStyle {
                    font: handles.get_font("base.ttf"),
                    font_size: 52.0,
                    color: Color::BLACK
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Right
                }
            ),
            transform: Transform::from_xyz(-512.0, 256.0, UI_TEXT),
            ..Default::default()
        }).id());
        entity_ids.push(commands.spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Settings",
                TextStyle {
                    font: handles.get_font("base.ttf"),
                    font_size: 52.0,
                    color: Color::BLACK
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Right
                }
            ),
            transform: Transform::from_xyz(-512.0, -256.0, UI_TEXT),
            ..Default::default()
        }).id());
        entity_ids.push(commands.spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Quit",
                TextStyle {
                    font: handles.get_font("base.ttf"),
                    font_size: 52.0,
                    color: Color::BLACK
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Right
                }
            ),
            transform: Transform::from_xyz(512.0, -256.0, UI_TEXT),
            ..Default::default()
        }).id());
        entity_ids.push(commands.spawn_bundle(SpriteBundle {
            material: materials.add(handles.get_texture("gblin_exp_2.png").into()),
            ..Default::default()
        }).id());
        entity_ids.push(commands.spawn_bundle(SpriteBundle {
            material: materials.add(handles.get_texture("ts.png").into()),
            ..Default::default()
        }).id());
        entity_ids.push(commands.spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "\u{f790}",
                TextStyle {
                    font: handles.get_font("KreativeSquare.ttf"),
                    font_size: 34.0,
                    color: Color::BLACK
                },
                TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Right
                }
            ),
            transform: Transform::from_xyz(0.0, 0.0, CURSOR),
            ..Default::default()
        }).insert(CursorMarker {}).id());
        commands.spawn().insert(TitleScreenManager::new(entity_ids));
    }
}

pub fn title_screen_buttons(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    query_manager: Query<&mut TitleScreenManager>,
    query_cursor: Query<&mut Transform, With<CursorMarker>>,
    query_final: Query<Entity, With<TitleScreenManager>>,
    mousein: Res<Input<MouseButton>>,
    mut quit: EventWriter<bevy::app::AppExit>
) {
    if state.eq(&GameState::TitleScreen) {
        query_cursor.for_each_mut(|location| {
            if mousein.just_pressed(MouseButton::Left) {
                if
                    location.translation.x > PLAY_BUTTON_LOC.0 &&
                    location.translation.x < PLAY_BUTTON_LOC.0 + PLAY_BUTTON_SIZE.0 &&
                    location.translation.y > PLAY_BUTTON_LOC.1 &&
                    location.translation.y < PLAY_BUTTON_LOC.1 + PLAY_BUTTON_SIZE.1
                {
                    println!("Play button selected.");
                    query_manager.for_each_mut(|mut manager| {
                        manager.disassemble(&mut commands);
                    });
                    for entity in query_final.iter() {
                        commands.entity(entity).despawn();
                    }
                    state.change_state(GameState::Join);
                }
                else if
                    location.translation.x > NEW_BUTTON_LOC.0 &&
                    location.translation.x < NEW_BUTTON_LOC.0 + NEW_BUTTON_SIZE.0 &&
                    location.translation.y > NEW_BUTTON_LOC.1 &&
                    location.translation.y < NEW_BUTTON_LOC.1 + NEW_BUTTON_SIZE.1
                {
                    println!("New button selected.");
                    query_manager.for_each_mut(|mut manager| {
                        manager.disassemble(&mut commands);
                    });
                    for entity in query_final.iter() {
                        commands.entity(entity).despawn();
                    }
                    state.change_state(GameState::NewPage);
                }
                else if
                    location.translation.x > SETTINGS_BUTTON_LOC.0 &&
                    location.translation.x < SETTINGS_BUTTON_LOC.0 + SETTINGS_BUTTON_SIZE.0 &&
                    location.translation.y > SETTINGS_BUTTON_LOC.1 &&
                    location.translation.y < SETTINGS_BUTTON_LOC.1 + SETTINGS_BUTTON_SIZE.1
                {
                    println!("Settings button selected.");
                    query_manager.for_each_mut(|mut manager| {
                        manager.disassemble(&mut commands);
                    });
                    for entity in query_final.iter() {
                        commands.entity(entity).despawn();
                    }
                    state.change_state(GameState::Settings);
                }
                else if
                    location.translation.x > QUIT_BUTTON_LOC.0 &&
                    location.translation.x < QUIT_BUTTON_LOC.0 + QUIT_BUTTON_SIZE.0 &&
                    location.translation.y > QUIT_BUTTON_LOC.1 &&
                    location.translation.y < QUIT_BUTTON_LOC.1 + QUIT_BUTTON_SIZE.1
                {
                    println!("Quit button selected, exiting program.");
                    quit.send(bevy::app::AppExit {});
                }
            }
        });
    }
}

const PLAY_BUTTON_LOC: (f32, f32) = (512.0, 256.0);
const PLAY_BUTTON_SIZE: (f32, f32) = (128.0, 32.0);
const NEW_BUTTON_LOC: (f32, f32) = (-512.0, 256.0);
const NEW_BUTTON_SIZE: (f32, f32) = (96.0, 32.0);
const SETTINGS_BUTTON_LOC: (f32, f32) = (-512.0, -256.0);
const SETTINGS_BUTTON_SIZE: (f32, f32) = (256.0, 32.0);
const QUIT_BUTTON_LOC: (f32, f32) = (512.0, -256.0);
const QUIT_BUTTON_SIZE: (f32, f32) = (128.0, 32.0);
