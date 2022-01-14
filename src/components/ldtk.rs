use bevy::prelude::Component;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct TileMarker {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct PlayerMarker {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct InGameTile {
    pub chunk: (isize, isize)
}
