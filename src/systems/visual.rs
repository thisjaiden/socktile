mod title_screen;
use bevy::prelude::shape::RegularPolygon;
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
mod animated_sprites;
pub use animated_sprites::*;

use crate::prelude::*;

// TODO: move to seperate file
pub fn loading_prog(
    progress: Option<Res<ProgressCounter>>,
    mut query: Query<(Entity, &mut Transform), With<LoadingScreenProgress>>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        let (_e, mut transform) = query.single_mut();
        let pd = (progress.done as f32) / (progress.total as f32);
        transform.scale.x = pd * 9.5;

        //warn!("Progress: {:?}", progress);
    }
}

pub fn clear_old(
    mut commands: Commands,
    mut ui: ResMut<UIManager>,
    query: Query<Entity, With<RemoveOnStateChange>>,
) {
    info!("Clearing `RemoveOnStateChange` entities...");
    // Despawn entities tagged with `RemoveOnStateChange`
    query.for_each(|e| {
        commands.entity(e).despawn();
    });
    // Clear all UI elements
    ui.reset_ui();
}

pub fn logo(
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: server.load("core/logo.png"),
            ..default()
        },
        RemoveOnStateChange {},
    ));
    commands.spawn((
        Text2dBundle {
            transform: Transform::from_xyz(0.0, -200.0, UI_TEXT),
            text: Text {
                sections: vec![
                    TextSection {
                        value: String::from("Pat Cat Games"),
                        style: TextStyle {
                            font: server.load("font/blooming_grove.ttf"),
                            font_size: 64.0,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: String::from("\nNow Loading..."),
                        style: TextStyle {
                            font: server.load("font/blooming_grove.ttf"),
                            font_size: 48.0,
                            color: Color::BLACK,
                        },
                    },
                ],
                alignment: TextAlignment::Center,
                linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter
            },
            ..default()
        },
        RemoveOnStateChange {},
    ));
    let square = RegularPolygon::new(50.0, 4);
    let mesh = meshes.add(Mesh::from(square));
    let color = materials.add(ColorMaterial::from(Color::BLACK));
    commands.spawn((
        ColorMesh2dBundle {
            mesh: mesh.clone().into(),
            material: color.clone(),
            transform: Transform::from_xyz(-500.0, -300.0, UI_IMG),
            ..default()
        },
        RemoveOnStateChange {},
    ));
    commands.spawn((
        ColorMesh2dBundle {
            mesh: mesh.clone().into(),
            material: color.clone(),
            transform: Transform::from_xyz(500.0, -300.0, UI_IMG),
            ..default()
        },
        RemoveOnStateChange {},
    ));
    let i_transform = Transform::from_xyz(0.0, -300.0, UI_IMG);
    //i_transform.rotate_z(std::f32::consts::PI/4.0);
    commands.spawn((
        ColorMesh2dBundle {
            mesh: mesh.into(),
            material: color,
            transform: i_transform,
            ..default()
        },
        LoadingScreenProgress {},
        RemoveOnStateChange {},
    ));
    state.set(GameState::Load);
}

#[derive(Clone, Copy, Component)]
pub struct LoadingScreenProgress;
