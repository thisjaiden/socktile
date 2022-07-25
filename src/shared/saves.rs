use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Hash, Component)]
pub struct User {
    pub username: String,
    pub tag: u16
}
