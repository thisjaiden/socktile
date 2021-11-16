use bevy::prelude::Color;

use crate::{components::GamePosition, resources::animator::DisplayModal};

use super::FrameDetails;

pub fn fitsnwifi(frame: usize) -> FrameDetails {
    match frame {
        0 => {
            FrameDetails {
                location: GamePosition { x: 0.0, y: 0.0 },
                display_modals: vec![
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("This dev build REQUIRES a WIFI connection.")
                        ),
                        GamePosition { x: 0.0, y: 0.0 },
                        0
                    )
                ]
            }
        }
        1 | 2 => {
            FrameDetails {
                location: GamePosition { x: 0.0, y: 0.0 },
                display_modals: vec![
                    (
                        DisplayModal::NoUpdate,
                        GamePosition { x: 0.0, y: 0.0 },
                        0
                    )
                ]
            }
        }
        _ => unimplemented!()
    }
}
