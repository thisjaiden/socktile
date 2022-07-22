use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::{components::GamePosition, server::npc::Npc};

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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
/// Represents the type of a game object.
pub enum ObjectType {
    Tree,
    GroundItem(Item),
    Npc(Npc)
}

impl ObjectType {
    pub fn collider(&self) -> Option<(f64, f64)> {
        match self {
            Self::Tree => Some((64.0, 64.0)),
            Self::Npc(_who) => Some((64.0, 64.0)),
            _ => None
        }
    }
}
