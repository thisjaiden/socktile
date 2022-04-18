use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// Represents the state of a single tile of terrain.
pub struct TerrainState {
    pub tileset: usize,
    pub tile: usize
}

impl TerrainState {
    pub fn collides(&mut self, player: (f64, f64), offset_x: f64, offset_y: f64) -> bool {
        // TODO: properly define player hitbox beyond arbitrary numbers here
        self.collider_type().does_collide_with((player.0 - 32.0, player.1 - 28.0, 64.0, 64.0), offset_x, offset_y)
    }
    fn collider_type(&mut self) -> ColliderType {
        match self.tileset {
            58 => {
                match self.tile {
                    0 => ColliderType::TopLeft,
                    1 => ColliderType::Top,
                    2 => ColliderType::TopRight,
                    3 => ColliderType::InverseTopLeft,
                    4 => ColliderType::InverseTopRight,
                    8 => ColliderType::Left,
                    10 => ColliderType::Right,
                    11 => ColliderType::InverseBottomLeft,
                    12 => ColliderType::InverseBottomRight,
                    16 => ColliderType::BottomLeft,
                    17 => ColliderType::Bottom,
                    18 => ColliderType::BottomRight,
                    9 | 19 | 24..=28 | 32 | 34..=36 | 40..=42 => ColliderType::None,
                    n => panic!("Invalid tile in generic tileset! ({}:{})", self.tileset, n)
                }
            }
            n => panic!("Unknown tileset! ({n})")
        }
    }
}

#[derive(Debug)]
pub enum ColliderType {
    // No collider
    None,
    // Thin colliders prevent movement across the respective sides of the tile
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
    InverseTopLeft,
    InverseTopRight,
    InverseBottomLeft,
    InverseBottomRight,
}

impl ColliderType {
    fn collider_dimensions(&mut self) -> &[(f64, f64, f64, f64)] {
        match self {
            Self::None => &[],
            Self::TopLeft => &[(26.0, 0.0, 6.0, 32.0), (32.0, 32.0, 32.0, 6.0)],
            Self::Top => &[(0.0, 32.0, 64.0, 6.0)],
            Self::TopRight => &[(0.0, 32.0, 32.0, 6.0), (32.0, 0.0, 6.0, 32.0)],
            Self::Left => &[(26.0, 0.0, 6.0, 64.0)],
            Self::Right => &[(32.0, 0.0, 6.0, 64.0)],
            Self::BottomLeft => &[(26.0, 32.0, 6.0, 32.0), (32.0, 26.0, 32.0, 6.0)],
            Self::Bottom => &[(0.0, 26.0, 64.0, 6.0)],
            Self::BottomRight => &[(0.0, 26.0, 32.0, 6.0), (32.0, 32.0, 6.0, 32.0)],
            Self::InverseTopLeft => &[(32.0, 0.0, 6.0, 32.0), (32.0, 26.0, 32.0, 6.0)],
            Self::InverseTopRight => &[(0.0, 26.0, 32.0, 6.0), (26.0, 0.0, 6.0, 32.0)],
            Self::InverseBottomLeft => &[(32.0, 32.0, 32.0, 6.0), (32.0, 32.0, 6.0, 32.0)],
            Self::InverseBottomRight => &[(0.0, 32.0, 32.0, 6.0), (26.0, 32.0, 6.0, 32.0)]
        }
    }
    fn cube_colliders(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
        a.0 < (b.0 + b.2) &&
        (a.0 + a.2) > b.0 &&
        (a.1 + a.3) > b.1 &&
        a.1 < (b.1 + b.3)
    }
    pub fn does_collide_with(&mut self, other: (f64, f64, f64, f64), offset_x: f64, offset_y: f64) -> bool {
        let mut checks = vec![];
        for collider in self.collider_dimensions() {
            checks.push(Self::cube_colliders(
                (
                    collider.0 + offset_x,
                    collider.1 + offset_y,
                    collider.2,
                    collider.3
                ),
                other
            ));
        }
        checks.contains(&true)
    }
}
