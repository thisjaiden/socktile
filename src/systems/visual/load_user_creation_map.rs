use bevy::prelude::*;
use crate::{assets::FontAssets, components::TextBox, resources::ui::UIManager};

pub fn load_user_creation_map(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    uiman: ResMut<UIManager>
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: String::new(),
                    style: TextStyle {
                        font: font_assets.simvoni.clone(),
                        font_size: 35.0,
                        color: Color::BLACK
                    }
                }
                ;2 // we make two text components to show errors below if applicable
            ],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center
            }
        },
        ..Default::default()
    }).insert(TextBox {});
}
