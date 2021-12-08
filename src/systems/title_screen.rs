use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use crate::{components::CursorMarker, layers::{CURSOR, DEBUG}, resources::{Animation, Animator, AssetHandles, ConnectionStatus, Netty}, shared::{netty::Packet, saves::user}};

pub fn title_screen_spawner(
    mut commands: Commands,
    mut handles: ResMut<AssetHandles>,
    mut animator: ResMut<Animator>,
    mut netty: ResMut<Netty>
) {
        if netty.connection() == ConnectionStatus::NoInternet {
            // No internet - show error indefinitely.
            animator.request_animation(Animation::FloatInTitleScreenNoWIFI, true);
            // Remove all UI and handlers
            state.change_state(GameState::Void);
        }
        else if netty.connection() == ConnectionStatus::NoGGS {
            // No GGS - show error indefinitely.
            animator.request_animation(Animation::FloatInTitleScreenNoGGS, true);
            // Remove all UI and handlers
            state.change_state(GameState::Void);
        }
        else if user().is_none() {
            state.change_state(GameState::CreateUser);
            return;
        }
        else {
            netty.say(Packet::UserPresence(user().unwrap()));
            let fiid = animator.request_named_animation(Animation::FloatInTitleScreen, false, "tsbob");
            animator.request_animation_followup(fiid, Animation::TitleScreenBob, true);
        }
        commands.spawn_bundle(Text2dBundle {
            text: Text::with_section(
                '\u{f790}',
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
        }).insert(CursorMarker {});
    }
}

pub fn title_screen_buttons(
    mut state: ResMut<GameState>,
    query_cursor: Query<&mut Transform, With<CursorMarker>>,
    mousein: Res<Input<MouseButton>>,
    mut quit: EventWriter<bevy::app::AppExit>,
    mut lines: ResMut<DebugLines>,
    mut animator: ResMut<Animator>
) {
    if state.eq(&GameState::TitleScreen) {
        debug_box(&mut lines, PLAY_BUTTON_LOC, PLAY_BUTTON_SIZE);
        debug_box(&mut lines, NEW_BUTTON_LOC, NEW_BUTTON_SIZE);
        debug_box(&mut lines, SETTINGS_BUTTON_LOC, SETTINGS_BUTTON_SIZE);
        debug_box(&mut lines, QUIT_BUTTON_LOC, QUIT_BUTTON_SIZE);
        query_cursor.for_each_mut(|location| {
            if mousein.just_pressed(MouseButton::Left) {
                if
                    location.translation.x > PLAY_BUTTON_LOC.0 &&
                    location.translation.x < PLAY_BUTTON_LOC.0 + PLAY_BUTTON_SIZE.0 &&
                    location.translation.y > PLAY_BUTTON_LOC.1 &&
                    location.translation.y < PLAY_BUTTON_LOC.1 + PLAY_BUTTON_SIZE.1
                {
                    println!("Play button selected.");
                    animator.request_animation_end("tsbob");
                    state.change_state(GameState::Join);
                }
                else if
                    location.translation.x > NEW_BUTTON_LOC.0 &&
                    location.translation.x < NEW_BUTTON_LOC.0 + NEW_BUTTON_SIZE.0 &&
                    location.translation.y > NEW_BUTTON_LOC.1 &&
                    location.translation.y < NEW_BUTTON_LOC.1 + NEW_BUTTON_SIZE.1
                {
                    println!("New button selected.");
                    animator.request_animation_end("tsbob");
                    state.change_state(GameState::New);
                }
                else if
                    location.translation.x > SETTINGS_BUTTON_LOC.0 &&
                    location.translation.x < SETTINGS_BUTTON_LOC.0 + SETTINGS_BUTTON_SIZE.0 &&
                    location.translation.y > SETTINGS_BUTTON_LOC.1 &&
                    location.translation.y < SETTINGS_BUTTON_LOC.1 + SETTINGS_BUTTON_SIZE.1
                {
                    println!("Settings button selected.");
                    animator.request_animation_end("tsbob");
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

fn debug_box(
    lines: &mut ResMut<DebugLines>,
    pos: (f32, f32),
    size: (f32, f32)
) {
    lines.line_colored(
        Vec3::from((pos.0, pos.1, DEBUG)),
        Vec3::from((pos.0 + size.0, pos.1, DEBUG)),
         0.0, Color::RED
    );
    lines.line_colored(
        Vec3::from((pos.0, pos.1, DEBUG)),
        Vec3::from((pos.0, pos.1 + size.1, DEBUG)),
         0.0, Color::RED
    );
    lines.line_colored(
        Vec3::from((pos.0 + size.0, pos.1, DEBUG)),
        Vec3::from((pos.0 + size.0, pos.1 + size.1, DEBUG)),
         0.0, Color::RED
    );
    lines.line_colored(
        Vec3::from((pos.0, pos.1 + size.1, DEBUG)),
        Vec3::from((pos.0 + size.0, pos.1 + size.1, DEBUG)),
         0.0, Color::RED
    );
}

const PLAY_BUTTON_LOC: (f32, f32) = (440.0, 15.0);
const PLAY_BUTTON_SIZE: (f32, f32) = (128.0, 60.0);
const NEW_BUTTON_LOC: (f32, f32) = (440.0, 75.0);
const NEW_BUTTON_SIZE: (f32, f32) = (96.0, 60.0);
const SETTINGS_BUTTON_LOC: (f32, f32) = (-625.0, -90.0);
const SETTINGS_BUTTON_SIZE: (f32, f32) = (256.0, 60.0);
const QUIT_BUTTON_LOC: (f32, f32) = (-625.0, -150.0);
const QUIT_BUTTON_SIZE: (f32, f32) = (128.0, 60.0);
