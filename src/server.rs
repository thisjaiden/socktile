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
/// Represents one user's profile.
pub struct Profile {
    pub user: User,
    pub avalable_games: Vec<usize>,
}

/// Returns all profiles from the disk.
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

pub fn saves() -> Vec<SaveGame> {
    let mut saved_games = vec![];
    for file in std::fs::read_dir(save_folder()).expect("Unable to access saves.") {
        let wrkabl = file.unwrap().path();
        if wrkabl.extension().expect("File had no extension.") == "bic" {
            saved_games.push(
                bincode::deserialize(&std::fs::read(wrkabl).expect("Unable to read a save file."))
                    .expect("Encountered a courrupted save file."),
            );
        }
    }
    saved_games
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
