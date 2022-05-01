use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::components::GamePosition;

use super::player::Item;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Component)]
/// Represents a single game object.
pub struct Object {
    pub pos: GamePosition,
    pub rep: ObjectType,
    pub uuid: uuid::Uuid
}

impl Object {
    pub fn update(&mut self, updated: Object) {
        self.pos = updated.pos;
        self.rep = updated.rep;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// Represents the type of a game object.
pub enum ObjectType {
    Tree,
    GroundItem(Item)
}
