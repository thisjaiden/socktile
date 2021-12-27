#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TileMarker {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayerMarker {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InGameTile {
    pub chunk: (isize, isize),
    pub location: (usize, usize)
}
