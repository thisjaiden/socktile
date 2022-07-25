use crate::prelude::*;
use crate::resources::ui::SettingsPage;

pub fn clear_settings(
    mut commands: Commands,
    mut ui: ResMut<UIManager>,
    query: Query<Entity, With<SettingsPageComp>>
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
) {
    if man.settings_page == SettingsPage::Video && !man.on_page {
        man.on_page = true;
        man.add_ui(UIClickable {
            action: UIClickAction::IncreaseWindowScaling,
            location: (0.0, 0.0), // TODO
            size: (0.0, 0.0), // TODO
            removed_on_use: false,
            tag: Some(String::from("Settings"))
        });
        man.add_ui(UIClickable {
            action: UIClickAction::DecreaseWindowScaling,
            location: (0.0, 0.0), // TODO
            size: (0.0, 0.0), // TODO
            removed_on_use: false,
            tag: Some(String::from("Settings"))
        });
        man.add_ui(UIClickable {
            action: UIClickAction::ToggleVSync,
            location: (0.0, 0.0), // TODO
            size: (0.0, 0.0), // TODO
            removed_on_use: false,
            tag: Some(String::from("Settings"))
        });
        // Background
        commands.spawn_bundle(SpriteBundle {
            texture: core.video_settings.clone(),
            ..default()
        }).insert(SettingsPageComp { type_: 0 });
        // Leave button/text
        man.add_ui(UIClickable {
            action: UIClickAction::CloseSettings,
            location: (-600.0, -300.0),
            size: (150.0, 36.0),
            removed_on_use: true,
            tag: Some(String::from("Settings"))
        });
        commands.spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: String::from("Back"),
                    style: TextStyle {
                        font: fonts.simvoni.clone(),
                        font_size: 36.0,
                        color: Color::BLACK
                    }
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Left
                }
            },
            transform: Transform::from_xyz(-600.0, -300.0, UI_TEXT),
            ..default()
        }).insert(SettingsPageComp { type_: 0 });
        // Fullscreen button/text
        man.add_ui(UIClickable {
            action: UIClickAction::ToggleFullscreen,
            location: (0.0, 0.0),
            size: (260.0, 36.0),
            removed_on_use: false,
            tag: Some(String::from("Settings"))
        });
        commands.spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("Fullscreen: {}", disk.window_config().fullscreen),
                    style: TextStyle {
                        font: fonts.simvoni.clone(),
                        font_size: 36.0,
                        color: Color::BLACK
                    }
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Left
                }
            },
            transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
            ..default()
        }).insert(SettingsPageComp { type_: 1 });
    }
}
