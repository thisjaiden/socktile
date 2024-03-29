use crate::prelude::*;
use crate::resources::ui::SettingsPage;

pub fn clear_settings(
    mut commands: Commands,
    mut ui: ResMut<UIManager>,
    query: Query<Entity, With<SettingsPageComp>>,
) {
    // Despawn entities tagged with `SettingsPageComp`
    query.for_each(|e| {
        commands.entity(e).despawn();
    });
    // Clear settings UI elements
    ui.remove_tag("Settings");
    // Reset settings UI settings
    ui.on_page = false;
    ui.settings_page = SettingsPage::Video;
}

pub fn settings_video(
    mut commands: Commands,
    mut man: ResMut<UIManager>,
    disk: Res<Disk>,
    fonts: Res<FontAssets>,
    core: Res<CoreAssets>,
    lang_serve: Res<Assets<Language>>,
) {
    if man.settings_page == SettingsPage::Video && !man.on_page {
        let lang = lang_serve.get(&core.lang).unwrap();
        man.on_page = true;
        // Background
        commands.spawn((
            SpriteBundle {
                texture: core.video_settings.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 105.0),
                ..default()
            },
            SettingsPageComp { type_: 0 },
            RemoveOnStateChange,
        ));
        // Decrease scaling button/text
        man.add_ui(UIClickable {
            action: UIClickAction::DecreaseWindowScaling,
            location: (-45.0, -38.0),
            size: (20.0, 36.0),
            removed_on_use: false,
            tag: Some(String::from("Settings")),
        });
        commands.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: lang.get_key(".core.settings.scaling.decrease"),
                        style: TextStyle {
                            font: fonts.simvoni.clone(),
                            font_size: 36.0,
                            color: Color::BLACK,
                        },
                    }],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter
                },
                transform: Transform::from_xyz(-40.0, -38.0, UI_TEXT + 105.0),
                ..default()
            },
            SettingsPageComp { type_: 3 },
            RemoveOnStateChange,
        ));
        // Increase scaling button/text
        man.add_ui(UIClickable {
            action: UIClickAction::IncreaseWindowScaling,
            location: (35.0, -38.0),
            size: (20.0, 36.0),
            removed_on_use: false,
            tag: Some(String::from("Settings")),
        });
        commands.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: lang.get_key(".core.settings.scaling.increase"),
                        style: TextStyle {
                            font: fonts.simvoni.clone(),
                            font_size: 36.0,
                            color: Color::BLACK,
                        },
                    }],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter
                },
                transform: Transform::from_xyz(40.0, -38.0, UI_TEXT + 105.0),
                ..default()
            },
            SettingsPageComp { type_: 2 },
            RemoveOnStateChange,
        ));
        // Show scaling amount text
        commands.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: String::new(),
                        style: TextStyle {
                            font: fonts.simvoni.clone(),
                            font_size: 36.0,
                            color: Color::BLACK,
                        },
                    }],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter
                },
                transform: Transform::from_xyz(0.0, -38.0, UI_TEXT + 105.0),
                ..default()
            },
            SettingsPageComp { type_: 4 },
            RemoveOnStateChange,
        ));
        // Leave button/text
        man.add_ui(UIClickable {
            action: UIClickAction::CloseSettings,
            location: (-600.0, -300.0),
            size: (100.0, 36.0),
            removed_on_use: true,
            tag: Some(String::from("Settings")),
        });
        commands.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: lang.get_key(".core.settings.leave"),
                        style: TextStyle {
                            font: fonts.simvoni.clone(),
                            font_size: 36.0,
                            color: Color::BLACK,
                        },
                    }],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter
                },
                transform: Transform::from_xyz(-600.0, -300.0, UI_TEXT + 105.0),
                ..default()
            },
            SettingsPageComp { type_: 0 },
            RemoveOnStateChange,
        ));
        // Fullscreen button/text
        man.add_ui(UIClickable {
            action: UIClickAction::ToggleFullscreen,
            location: (0.0, 0.0),
            size: (315.0, 36.0),
            removed_on_use: false,
            tag: Some(String::from("Settings")),
        });
        let txtout: String = if disk.window_config().fullscreen {
            lang.get_key(".core.settings.fullscreen.on")
        }
        else {
            lang.get_key(".core.settings.fullscreen.off")
        };
        commands.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: txtout,
                        style: TextStyle {
                            font: fonts.simvoni.clone(),
                            font_size: 36.0,
                            color: Color::BLACK,
                        },
                    }],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter
                },
                transform: Transform::from_xyz(0.0, 0.0, UI_TEXT + 105.0),
                ..default()
            },
            SettingsPageComp { type_: 1 },
            RemoveOnStateChange,
        ));
    }
}
