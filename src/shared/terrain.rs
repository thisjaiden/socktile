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
        self.collider_type().is_colliding((player.0 - 32.0, player.1 - 28.0, 64.0, 64.0), offset_x, offset_y)
    }
    fn collider_type(&mut self) -> ColliderType {
        match self.tileset {
            58 => {
                match self.tile {
                    0 => ColliderType::GenericTopLeft,
                    1 => ColliderType::GenericTop,
                    2 => ColliderType::GenericTopRight,
                    3 => ColliderType::GenericInverseTopLeft,
                    4 => ColliderType::GenericInverseTopRight,
                    8 => ColliderType::GenericLeft,
                    10 => ColliderType::GenericRight,
                    11 => ColliderType::GenericInverseBottomLeft,
                    12 => ColliderType::GenericInverseBottomRight,
                    16 => ColliderType::GenericBottomLeft,
                    17 => ColliderType::GenericBottom,
                    18 => ColliderType::GenericBottomRight,
                    9 | 19 | 24..=28 | 32 | 34..=36 | 40..=42 => ColliderType::GenericNone,
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
    GenericNone,
    // Thin colliders prevent movement across the respective sides of the tile
    GenericTopLeft,
    GenericTop,
    GenericTopRight,
    GenericLeft,
    GenericRight,
    GenericBottomLeft,
    GenericBottom,
    GenericBottomRight,
    GenericInverseTopLeft,
    GenericInverseTopRight,
    GenericInverseBottomLeft,
    GenericInverseBottomRight,
}

impl ColliderType {
    fn collider_dimensions(&mut self) -> &[(f64, f64, f64, f64)] {
        match self {
            Self::GenericNone => return &[],
            Self::GenericTopLeft => return &[(26.0, 0.0, 6.0, 32.0), (32.0, 32.0, 32.0, 6.0)],
            Self::GenericTop => return &[(0.0, 32.0, 64.0, 6.0)],
            Self::GenericTopRight => return &[(0.0, 32.0, 32.0, 6.0), (32.0, 0.0, 6.0, 32.0)],
            Self::GenericLeft => return &[(26.0, 0.0, 6.0, 64.0)],
            Self::GenericRight => return &[(32.0, 0.0, 6.0, 64.0)],
            Self::GenericBottomLeft => return &[(26.0, 32.0, 6.0, 32.0), (32.0, 26.0, 32.0, 6.0)],
            Self::GenericBottom => return &[(0.0, 26.0, 64.0, 6.0)],
            Self::GenericBottomRight => return &[(0.0, 26.0, 32.0, 6.0), (32.0, 32.0, 6.0, 32.0)],
            Self::GenericInverseTopLeft => return &[(32.0, 0.0, 6.0, 32.0), (32.0, 26.0, 32.0, 6.0)],
            Self::GenericInverseTopRight => return &[(0.0, 26.0, 32.0, 6.0), (26.0, 0.0, 6.0, 32.0)],
            Self::GenericInverseBottomLeft => return &[(32.0, 32.0, 32.0, 6.0), (32.0, 32.0, 6.0, 32.0)],
            Self::GenericInverseBottomRight => return &[(0.0, 32.0, 32.0, 6.0), (26.0, 32.0, 6.0, 32.0)]
        }
    }
    fn cube_colliders(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
        a.0 < (b.0 + b.2) &&
        (a.0 + a.2) > b.0 &&
        (a.1 + a.3) > b.1 &&
        a.1 < (b.1 + b.3)
    }
    pub fn is_colliding(&mut self, other: (f64, f64, f64, f64), offset_x: f64, offset_y: f64) -> bool {
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
        return checks.contains(&true)
    }
}
