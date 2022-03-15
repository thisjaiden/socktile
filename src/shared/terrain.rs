use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// Represents the state of a single tile of terrain.
pub struct TerrainState {
    pub tileset: usize,
    pub tile: usize
}

impl TerrainState {
    pub fn collides(&mut self, other: (f32, f32, f32, f32)) -> bool {
        self.collider_type().is_colliding(other)
    }
    fn collider_type(&mut self) -> ColliderType {
        match self.tileset {
            0 => {
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
                    _ => panic!("Invalid tile in generic tileset!")
                }
            }
            _ => panic!("Unknown tileset!")
        }
    }
}

pub enum ColliderType {
    // No collider
    GenericNone,
    // Whole object is a collider
    GenericAll,
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
    fn collider_dimensions(&mut self) -> &[(f32, f32, f32, f32)] {
        match self {
            Self::GenericNone => return &[(0.0, 0.0, 0.0, 0.0)],
            Self::GenericAll => return &[(0.0, 0.0, 64.0, 64.0)],
            Self::GenericTopLeft => return &[(26.0, 0.0, 6.0, 64.0), (32.0, 32.0, 32.0, 6.0)],
            Self::GenericTop => return &[(0.0, 32.0, 64.0, 6.0)],
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
