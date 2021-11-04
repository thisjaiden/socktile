use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use super::{listing::GameListing, world::World};

pub fn save_folder() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap();
    dir.push("saves");
    std::fs::create_dir_all(dir.clone()).unwrap();
    dir
}

pub fn profile_folder() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap();
    dir.push("users");
    std::fs::create_dir_all(dir.clone()).unwrap();
    dir
}

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

pub fn profiles() -> Vec<Profile> {
    let mut saved_users = vec![];
    for file in std::fs::read_dir(profile_folder()).expect("Unable to access profiles.").into_iter() {
        let wrkabl = file.unwrap().path();
        if wrkabl.extension().unwrap() == "bic" {
            saved_users.push(
                bincode::deserialize(&std::fs::read(wrkabl).expect("Unable to read a profile.")).expect("Encountered a courrupted profile.")
            );
        }
    }
    saved_users
}

pub fn saves() -> Vec<SaveGame> {
    let mut saved_games = vec![];
    for file in std::fs::read_dir(save_folder()).expect("Unable to access saves.") {
        let wrkabl = file.unwrap().path();
        if wrkabl.extension().unwrap() == "bic" {
            saved_games.push(
                bincode::deserialize(&std::fs::read(wrkabl).expect("Unable to read a save file.")).expect("Encountered a courrupted save file.")
            );
        }
    }
    saved_games
}

pub fn save(save: SaveGame) {
    let enc = bincode::serialize(&save).unwrap();
    std::fs::write(save.path, enc).unwrap();
}

pub fn save_user(user: User) {
    let enc = bincode::serialize(&user).unwrap();
    std::fs::write(user_location(), enc).unwrap();
}

pub fn save_profile(profile: Profile) {
    let enc = bincode::serialize(&profile).unwrap();
    let mut path = profile_folder();
    path.push(format!("{}{}.bic", profile.clone().user.username, profile.clone().user.tag));
    std::fs::write(path, enc).unwrap();
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SaveGame {
    pub public_name: String,
    pub internal_name: String,
    pub version: String,
    pub data: World,
    pub path: PathBuf,
    pub whitelist: Option<Vec<User>>,
    pub blacklist: Vec<User>,
    pub played_before: Vec<User>,
    pub owner: User,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub tag: u16
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Profile {
    pub user: User,
    pub joined_games: Vec<GameListing>,
    pub invited_games: Vec<GameListing>,
    pub created_games: Vec<GameListing>
}
