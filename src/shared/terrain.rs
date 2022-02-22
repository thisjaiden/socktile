use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// Represents the state of a single tile of terrain.
pub enum TerrainState {
    // tileset_example
    TEFullGround,
    TEBottomLine,
    TELeftBottomLine,
    TELeftLine,
    TEWater,
    TEBottomLeftCorner,
    TEWall,
    TERightLine
}

impl TerrainState {
    pub fn from_tile(tileset: usize, tile: usize) -> TerrainState {
        match tileset {
            0 => {
                match tile {
                    0 => return Self::TEFullGround,
                    1 => return Self::TEBottomLine,
                    2 => return Self::TELeftBottomLine,
                    3 => return Self::TELeftLine,
                    4 => return Self::TEWater,
                    5 => return Self::TEBottomLeftCorner,
                    6 => return Self::TEWall,
                    7 => return Self::TERightLine,
                    _ => panic!("an unregistered tile was used.")
                }
            }
            _ => panic!("an unregistered tileset was used.")
        }
    }
    pub fn collides(&mut self, other: (f32, f32, f32, f32)) -> bool {
        self.collider_type().is_colliding(other)
    }
    fn collider_type(&mut self) -> ColliderType {
        match self {
            Self::TEFullGround => ColliderType::None,
            Self::TEBottomLine => ColliderType::DownThin,
            Self::TELeftBottomLine => ColliderType::LeftDownThin,
            Self::TELeftLine => ColliderType::LeftThin,
            Self::TEWater => ColliderType::All,
            Self::TEBottomLeftCorner => ColliderType::None,
            Self::TEWall => ColliderType::All,
            Self::TERightLine => ColliderType::RightThin
        }
    }
}

pub enum ColliderType {
    // No collider
    None,
    // Whole object is a collider
    All,
    // Thin colliders prevent movement across the respective sides of the tile
    DownThin,
    LeftThin,
    RightThin,
    UpThin,
    LeftDownThin,
    RightDownThin,
    UpDownThin,
    LeftUpThin,
    LeftRightThin,
    RightUpThin,
}

impl ColliderType {
    fn collider_dimensions(&mut self) -> &[(f32, f32, f32, f32)] {
        match self {
            Self::None => return &[(0.0, 0.0, 0.0, 0.0)],
            Self::All => return &[(0.0, 0.0, 64.0, 64.0)],
            Self::DownThin => return &[(0.0, 0.0, 64.0, 1.0)],
            Self::LeftThin => return &[(0.0, 0.0, 1.0, 64.0)],
            Self::RightThin => return &[(63.0, 0.0, 1.0, 64.0)],
            Self::UpThin => return &[(0.0, 63.0, 64.0, 1.0)],
            Self::LeftDownThin => return &[(0.0, 0.0, 64.0, 1.0), (0.0, 0.0, 1.0, 64.0)],
            Self::RightDownThin => return &[(63.0, 0.0, 1.0, 64.0), (0.0, 0.0, 64.0, 1.0)],
            _ => todo!()
        }
    }
    fn cube_colliders(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> bool {
        a.0 < (b.0 + b.2) &&
        (a.0 + a.2) > b.0 &&
        (a.1 + a.3) > b.1 &&
        a.1 < (b.1 + b.3)
    }
    pub fn is_colliding(&mut self, other: (f32, f32, f32, f32)) -> bool {
        let mut checks = vec![];
        for collider in self.collider_dimensions() {
            checks.push(Self::cube_colliders(*collider, other));
        }
        return checks.contains(&true)
    }
}
