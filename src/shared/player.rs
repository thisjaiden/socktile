use serde::{Deserialize, Serialize};

use crate::components::GamePosition;

use super::saves::User;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Player {
    pub user: User,
    pub location: GamePosition
}
