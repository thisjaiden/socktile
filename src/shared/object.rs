use serde::{Deserialize, Serialize};

use crate::components::GamePosition;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
/// Represents a single game object.
pub struct Object {
    pub pos: GamePosition,
    pub rep: ObjectType,
    pub id: usize
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ObjectType {

}
