use crate::prelude::*;
use std::time::Duration;
use bevy_easings::{Ease, EaseFunction};

pub fn title_screen(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    core: Res<CoreAssets>,
    core_serve: Res<Assets<ModularAssets>>,
    mut ui: ResMut<UIManager>
) {
    let core_assets = core_serve.get(&core.core).unwrap();
    commands.spawn_bundle(SpriteBundle {
        texture: core.title_screen.clone(),
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND),
        ..default()
    })
    .insert(RemoveOnStateChange {});
    // New game text
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: core_assets.get_lang("en_us.core.title_screen.new_game"),
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
        ..default()
    })
    .insert(RemoveOnStateChange {})
    .insert(
        Transform::from_xyz(0.0, -3000.0, UI_TEXT)
        .ease_to(
            Transform::from_xyz(512.0, 256.0, UI_TEXT),
            EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::Once { duration: Duration::from_millis(1500) }
        )
    );
    // Join game text
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: core_assets.get_lang("en_us.core.title_screen.join_game"),
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
        ..default()
    })
    .insert(RemoveOnStateChange {})
    .insert(
        Transform::from_xyz(0.0, -3000.0, UI_TEXT)
        .ease_to(
            Transform::from_xyz(-512.0, 256.0, UI_TEXT),
            EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::Once { duration: Duration::from_millis(2000) }
        )
    );
    // Quit game text
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: core_assets.get_lang("en_us.core.title_screen.quit"),
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
        ..default()
    })
    .insert(RemoveOnStateChange {})
    .insert(
        Transform::from_xyz(0.0, -3000.0, UI_TEXT)
        .ease_to(
            Transform::from_xyz(-512.0, -256.0, UI_TEXT),
            EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::Once { duration: Duration::from_millis(1000) }
        )
    );
    // Settings text
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: core_assets.get_lang("en_us.core.title_screen.settings"),
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
        ..default()
    })
    .insert(RemoveOnStateChange {})
    .insert(
        Transform::from_xyz(0.0, -3000.0, UI_TEXT)
        .ease_to(
            Transform::from_xyz(512.0, -256.0, UI_TEXT),
            EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::Once { duration: Duration::from_millis(500) }
        )
    );
    // Splash text
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: core_assets.get_lang("en_us.core.title_screen.splash"),
                    style: TextStyle {
                        font: font_assets.apple_tea.clone(),
                        font_size: 40.0,
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
            128.0,
            UI_TEXT
        ),
        ..default()
    })
    .insert(RemoveOnStateChange {})
    .insert(
        Transform::from_xyz(0.0, 128.0, UI_TEXT)
        .ease_to(
            Transform::from_xyz(0.0, 128.0, UI_TEXT).with_scale(Vec3::new(1.05, 1.05, 1.05)),
            EaseFunction::SineInOut,
            bevy_easings::EasingType::PingPong {
                duration: Duration::from_millis(800),
                pause: None
            }
        )
    );
    // player username in bottom left
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
        location: (-710.0, -210.0),
        size: (410.0, 100.0),
        ..default()
    });
    ui.add_ui(UIClickable {
        action: UIClickAction::OpenSettings,
        location: (330.0, -210.0),
        size: (350.0, 100.0),
        ..default()
    });
    ui.add_ui(UIClickable {
        action: UIClickAction::GoToCreateWorld,
        location: (230.0, 310.0),
        size: (560.0, 100.0),
        ..default()
    });
    ui.add_ui(UIClickable {
        action: UIClickAction::ViewWorldList,
        location: (-790.0, 310.0),
        size: (560.0, 100.0),
        ..default()
    });
}
