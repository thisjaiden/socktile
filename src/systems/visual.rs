mod title_screen;
use iyes_progress::ProgressCounter;
pub use title_screen::title_screen;
mod make_user;
pub use make_user::make_user;
mod update_title_screen_user;
pub use update_title_screen_user::*;
mod create_world;
pub use create_world::create_world;
mod join_world;
pub use join_world::join_world;
mod settings;
pub use settings::*;

use crate::prelude::*;

// TODO: move to seperate file
pub fn loading_prog(
    progress: Option<Res<ProgressCounter>>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        warn!("Progress: {:?}", progress);
    }
}

pub fn clear_old(
    mut commands: Commands,
    mut ui: ResMut<UIManager>,
    query: Query<Entity, With<RemoveOnStateChange>>
) {
    // Despawn entities tagged with `RemoveOnStateChange`
    query.for_each(|e| {
        commands.entity(e).despawn();
    });
    // Clear all UI elements
    ui.reset_ui();
}

pub fn logo(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    server: Res<AssetServer>
) {
    commands.spawn_bundle(SpriteBundle {
        texture: server.load("core/logo.png"),
        ..default()
    }).insert(RemoveOnStateChange {});
    commands.spawn_bundle(Text2dBundle {
        transform: Transform::from_xyz(0.0, -200.0, UI_TEXT),
        text: Text {
            sections: vec![
                TextSection {
                    value: String::from("Pat Cat Games"),
                    style: TextStyle {
                        font: server.load("font/blooming_grove.ttf"),
                        font_size: 64.0,
                        color: Color::BLACK
                    }
                },
                TextSection {
                    value: String::from("\nNow Loading..."),
                    style: TextStyle {
                        font: server.load("font/blooming_grove.ttf"),
                        font_size: 48.0,
                        color: Color::BLACK
                    }
                }
            ],
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center
            }
        },
        ..default()
    }).insert(RemoveOnStateChange {});
    state.overwrite_set(GameState::Load).unwrap();
}
