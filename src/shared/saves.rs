use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use super::world::World;


pub fn user_location() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap();
    dir.push("me.bic");
    dir
}

pub fn user() -> Option<User> {
    let dta = std::fs::read(user_location());
    match dta {
        Ok(data) => {
            Some(bincode::deserialize(&data).expect("Encountered a courrupted user."))
        }
        Err(_) => {
            println!("Unable to find me.bic, assuming there is no user profile.");
            None
        }
    }
}

pub fn save_user(user: User) {
    let enc = bincode::serialize(&user).unwrap();
    std::fs::write(user_location(), enc).unwrap();
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub tag: u16
}
