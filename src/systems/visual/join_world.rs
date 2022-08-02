use crate::prelude::*;

pub fn join_world(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    core: Res<CoreAssets>,
    core_serve: Res<Assets<ModularAssets>>,
    mut ui: ResMut<UIManager>
) {
    let core_assets = core_serve.get(&core.core).unwrap();
    // Background
    commands.spawn_bundle(SpriteBundle {
        texture: core.join_world.clone(),
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND),
        ..default()
    })
    .insert(RemoveOnStateChange {});
    // Cancel text
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: core_assets.get_lang("en_us.core.join_world.cancel"),
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
        transform: Transform::from_xyz(
            -600.0,
            -400.0,
            UI_TEXT
        ),
        ..default()
    })
    .insert(RemoveOnStateChange {});
    ui.add_ui(UIClickable {
        action: UIClickAction::GoToTitleScreen,
        location: (-700.0, -350.0),
        size: (200.0, 100.0),
        ..default()
    });
}
