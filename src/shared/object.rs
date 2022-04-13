use serde::{Deserialize, Serialize};

use crate::components::GamePosition;

use super::player::Item;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
/// Represents a single game object.
pub struct Object {
    pub pos: GamePosition,
    pub rep: ObjectType,
    pub uuid: uuid::Uuid
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// Represents the type of a game object.
pub enum ObjectType {
    Tree,
    GroundItem(Item)
}
