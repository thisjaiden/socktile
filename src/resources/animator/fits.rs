use bevy::prelude::Color;

use crate::{components::GamePosition, layers::BACKGROUND, resources::animator::DisplayModal};

use super::FrameDetails;

pub fn fits(frame: usize) -> FrameDetails {
    match frame {
        0 | 1 => {
            FrameDetails {
                location: GamePosition { x: 0.0, y: 0.0 },
                display_modals: vec![
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("Join")
                        ),
                        GamePosition { x: 0.0, y: 0.0 },
                        0
                    ),
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("New")
                        ),
                        GamePosition { x: 0.0, y: 0.0 },
                        1
                    ),
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("Exit")
                        ),
                        GamePosition { x: 0.0, y: 0.0 },
                        2
                    ),
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("Settings")
                        ),
                        GamePosition { x: 0.0, y: 0.0 },
                        3
                    ),
                    (
                        DisplayModal::Sprite(
                            String::from("title_screen_background.png"),
                            BACKGROUND
                        ),
                        GamePosition { x: 0.0, y: 0.0 },
                        4
                    )
                ]
            }
        }
        i => {
            FrameDetails {
                location: GamePosition { x: 0.0, y: 0.0 },
                display_modals: vec![
                    (
                        // https://www.desmos.com/calculator/f1zarrftbo
                        // https://www.desmos.com/calculator/phg9dopwzv
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("Exit")
                        ),
                        GamePosition {
                            x: i as f64 - 1000.0,
                            y: (i as f64 - 800.0 * ((i as f64 / 50.0).sin() * 0.2)) - 400.0
                        },
                        2
                    ),
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("Settings")
                        ),
                        GamePosition {
                            x: i as f64 - 1000.0,
                            y: (i as f64 - 800.0 * ((i as f64 / 50.0).sin() * 0.2)) - 350.0
                        },
                        3
                    )
                ]
            }
        }
    }
}
