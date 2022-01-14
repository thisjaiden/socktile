use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub struct GamePosition {
    pub x: f64,
    pub y: f64
}
