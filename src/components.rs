use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
/// A Text2dBundle with TextBox inserted will display the contents of the TextBox resource.
pub struct TextBox {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
/// An Entity with both a Transform and a CursorMarker will be moved to the cursor's location.
pub struct CursorMarker {}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
// todo: move this
pub struct GamePosition {
    pub x: f64,
    pub y: f64
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct PauseMenuMarker {}

pub mod ldtk {
    use crate::shared::saves::User;
    use bevy::prelude::Component;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
    pub struct TileMarker {}

    #[derive(Clone, Debug, Eq, PartialEq, Component, Hash)]
    pub struct PlayerMarker {
        pub user: User,
        pub isme: bool
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
    pub struct InGameTile {
        pub chunk: (isize, isize)
    }
}
