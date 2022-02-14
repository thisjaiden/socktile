use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Hash)]
pub struct User {
    pub username: String,
    pub tag: u16
}
