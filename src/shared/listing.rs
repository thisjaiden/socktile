use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GameListing {
    pub public_name: String,
    pub description: String,
    pub internal_id: usize,
    pub local: bool,
    pub address: String,
    pub password: bool,
    pub played: bool
}
