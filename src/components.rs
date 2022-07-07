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

impl GamePosition {
    pub fn zero() -> GamePosition {
        GamePosition { x: 0.0, y: 0.0 }
    }
    pub fn distance(&self, other: GamePosition) -> f64 {
        // d=√((x_2-x_1)²+(y_2-y_1)²)
        (((other.x - self.x).powi(2))+((other.y - self.y).powi(2))).sqrt()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct ChatBox {
    pub location: usize
}

#[derive(Clone, Copy, Debug, Component)]
pub struct UILocked {}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct PauseMenuMarker {
    pub type_: usize
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct HotbarMarker {
    pub location: usize,
    /// Represents the type of hotbar object this is.
    /// 1 = slot background
    /// 2 = selected slot
    /// 3 = slot contents
    pub type_: usize
}

#[derive(Clone, Copy, Debug, Component)]
/// Indicates a Text object that is used to show the user's username
/// on the titlescreen in the bottom left.
pub struct TitleScreenUser {}

#[derive(Clone, Copy, Debug, Component)]
pub struct SettingsPageComp {
    /// what component this is
    /// 0 | unimportant/misc
    /// 1 | video settings fullscreen text
    pub type_: u8
}

#[derive(Clone, Copy, Debug, Component)]
pub struct RemoveOnStateChange {}

pub mod ldtk {
    use crate::{shared::saves::User, modular_assets::TransitionType};
    use bevy::prelude::Component;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
    pub struct TileMarker {}

    #[derive(Clone, Debug, Eq, PartialEq, Component, Hash)]
    pub struct PlayerMarker {
        pub user: User,
        pub isme: bool
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
    pub struct Tile {
        pub chunk: (isize, isize),
        pub position: (usize, usize),
        pub transition_type: TransitionType
    }
}
