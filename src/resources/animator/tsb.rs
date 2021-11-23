use bevy::prelude::Color;

use crate::{components::GamePosition, layers::BACKGROUND, resources::animator::DisplayModal};

use super::FrameDetails;

pub fn tsb(frame: usize) -> FrameDetails {
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
                            String::from("Join")
                        ),
                        GamePosition { x: 128.0, y: 0.0 },
                        0
                    ),
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("New")
                        ),
                        GamePosition { x: 128.0, y: 50.0 },
                        1
                    ),
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("Exit")
                        ),
                        GamePosition { x: -128.0, y: 128.0 },
                        2
                    ),
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("Settings")
                        ),
                        GamePosition { x: -128.0, y: -128.0 },
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
            // prev at 400
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
                        GamePosition {
                            x: 450.0,
                            y: 17.0 + ((i as f64 / 50.0).sin() * 3.0)
                        },
                        0
                    ),
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("New")
                        ),
                        GamePosition {
                            x: 450.0,
                            y: 67.0 + ((i as f64 / 50.0).sin() * 3.0)
                        },
                        1
                    ),
                    (
                        DisplayModal::Text(
                            String::from("base.ttf"),
                            64.0,
                            Color::BLACK,
                            String::from("Exit")
                        ),
                        GamePosition {
                            x: -600.0,
                            y: -160.0 + ((i as f64 / 50.0).sin() * 3.0)
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
                            x: -600.0,
                            y: -110.0 + ((i as f64 / 50.0).sin() * 3.0)
                        },
                        3
                    )
                ]
            }
        }
    }
}