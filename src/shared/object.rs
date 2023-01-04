use crate::prelude::*;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Component)]
/// Represents a single game object.
pub struct Object {
    pub pos: Transform,
    pub rep: ObjectType,
    pub uuid: uuid::Uuid,
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
    Tree(usize),
    GroundItem(Item),
    Npc(Npc),
}

impl ObjectType {
    pub fn collider(&self) -> Option<(f32, f32)> {
        match self {
            Self::Tree(_str) => Some((64.0, 64.0)),
            Self::Npc(_who) => Some((64.0, 64.0)),
            _ => None,
        }
    }
}
