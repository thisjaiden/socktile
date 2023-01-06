use std::{net::SocketAddr, path::PathBuf};

use crate::prelude::*;
use bevy::utils::HashMap;

mod handler;
use handler::handler;
mod tick;
use tick::tick;

mod globals;
pub use globals::Globals;

use self::tick::{profile_folder, save_folder};

pub mod npc;
mod world;

/// Starts the game server!
#[cfg(not(target_arch = "wasm32"))]
pub fn startup(_arguments: Vec<String>) -> ! {
    // TODO: add argument functionality back
    netty::server::launch_server::<Packet, Globals>(netty::server::ServerConfig {
        public_facing: true,
        tcp_port: TCP_PORT,
        ws_port: WS_PORT,
        handler,
        tick,
        ..default()
    });
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
/// Represents one user's profile with the server
pub struct Profile {
    /// This player's username and tag
    pub user: User,
    /// A list of all world ids for which this player can join
    pub avalable_games: Vec<usize>,
}

/// Returns all [Profile]s from the disk.
/// 
/// # Panics
/// This function can panic if there is a faliure reading the directory given by [profile_folder]
/// 
/// This function can panic if a file in the above directory has no file extension, cannot be read,
/// or cannot be deserialized
pub fn profiles() -> Vec<Profile> {
    let mut saved_users = vec![];
    for file in std::fs::read_dir(profile_folder()).expect("Unable to access profiles.") {
        let wrkabl = file.unwrap().path();
        if wrkabl.extension().expect("File had no extension.") == "bic" {
            saved_users.push(
                bincode::deserialize(&std::fs::read(wrkabl).expect("Unable to read a profile."))
                    .expect("Encountered a courrupted profile."),
            );
        }
    }
    saved_users
}

/// Returns all [SaveGame]s from the disk.
/// 
/// # Errors
/// This function can return an error if there is a faliure reading the directory given by
/// [save_folder]
/// 
/// This function can return an error if a file in the above directory has no file extension, cannot
/// be read, or cannot be deserialized
pub fn get_disk_savegames() -> Result<Vec<SaveGame>, anyhow::Error> {
    use std::ffi::OsStr;

    // A list that will be returned with all loaded games on completion
    let mut loaded_saves = vec![];
    // Read the directory containing save files
    let directory = std::fs::read_dir(save_folder())?;
    // For each file (save) in this directory...
    for pfile in directory {
        let file = pfile?;
        // If the file ends with a .bic extension, it's probably a save
        if file.path().extension() == Some(OsStr::new("bic")) {
            // Add the deserialized data into our list of loaded games
            loaded_saves.push(bincode::deserialize(&std::fs::read(file.path())?)?);
        }
    }
    // Return everything we've collected!
    return Ok(loaded_saves);
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SaveGame {
    pub public_name: String,
    pub internal_id: usize,
    pub data: world::World,
    pub path: PathBuf,
    pub whitelist: Vec<User>,
    pub played_before: Vec<User>,
    pub owner: User,
}
