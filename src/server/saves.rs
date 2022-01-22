use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::shared::{saves::User, world::World};

/// Returns a `PathBuf` to the folder used for storing profiles.
pub fn profile_folder() -> PathBuf {
    let mut dir = std::env::current_dir().expect("Unable to access the current directory.");
    dir.push("users");
    std::fs::create_dir_all(dir.clone()).expect("Unable to create required directories.");
    dir
}

/// Saves a `Profile` to the disk.
pub fn save_profile(profile: Profile) {
    // Encode profile
    let enc = bincode::serialize(&profile).expect("Unable to serialize a Profile.");

    // Get appropriate path and name
    let mut path = profile_folder();
    path.push(format!("{}{}.bic", profile.user.username, profile.user.tag));

    // Save to disk
    std::fs::write(path, enc).expect("Unable to write a profile to the disk.");
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
/// Represents one user's profile.
pub struct Profile {
    pub user: User,
    pub avalable_games: Vec<usize>
}

/// Returns all profiles from the disk.
pub fn profiles() -> Vec<Profile> {
    let mut saved_users = vec![];
    for file in std::fs::read_dir(profile_folder()).expect("Unable to access profiles.") {
        let wrkabl = file.unwrap().path();
        if wrkabl.extension().expect("File had no extension.") == "bic" {
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
        if wrkabl.extension().expect("File had no extension.") == "bic" {
            saved_games.push(
                bincode::deserialize(&std::fs::read(wrkabl).expect("Unable to read a save file.")).expect("Encountered a courrupted save file.")
            );
        }
    }
    saved_games
}

pub fn save(save: SaveGame) {
    let enc = bincode::serialize(&save).expect("Unable to serialize a SaveGame.");
    std::fs::write(save.path, enc).expect("Unable to write a SaveGame to disk.");
}

pub fn save_folder() -> PathBuf {
    let mut dir = std::env::current_dir().expect("Unable to access the current directory.");
    dir.push("saves");
    std::fs::create_dir_all(dir.clone()).expect("Unable to create required directories.");
    dir
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SaveGame {
    pub public_name: String,
    pub internal_id: usize,
    pub version: String,
    pub data: World,
    pub path: PathBuf,
    pub whitelist: Option<Vec<User>>,
    pub blacklist: Vec<User>,
    pub played_before: Vec<User>,
    pub owner: User,
}
