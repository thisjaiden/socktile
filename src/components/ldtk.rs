use bevy::prelude::Component;

use crate::shared::saves::User;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct TileMarker {}

#[derive(Clone, Debug, Eq, PartialEq, Component)]
pub struct PlayerMarker {
    pub user: User
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct InGameTile {
    pub chunk: (isize, isize)
}
