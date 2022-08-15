use std::{net::SocketAddr, path::PathBuf};

use bevy::utils::HashMap;
use crate::prelude::*;

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
    /*
                    Packet::JoinWorld(world_id) => {
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::JoinWorld(usize)");
                        
                        let mut world_index = 0;
                        for (index, world) in saves.iter().enumerate() {
                            if world.internal_id == world_id {
                                world_index = index;
                                break;
                            }
                        }
                        let mut player_info = None;
                        for (index, player) in saves[world_index].data.offline_players.clone().into_iter().enumerate() {
                            if &player.0 == owner {
                                player_info = Some(saves[world_index].data.offline_players.remove(index));
                                break;
                            }
                        }
                        if player_info == None {
                            player_info = Some((owner.clone(), GamePosition { x: 0.0, y: 0.0 }, PlayerData::new()));
                        }
                        let player_info = player_info.unwrap();
                        let mut other_players = vec![];
                        for (user, _, _) in saves[world_id].data.players.clone() {
                            let ip = ip_by_user.get(&user).expect("A user online on a server had no IP address");
                            other_players.push((Packet::PlayerConnected(owner.clone(), player_info.1), *ip));
                        }
                        if !saves[world_index].data.players.contains(&player_info) {
                            saves[world_index].data.players.push(player_info.clone());
                        }
                        else {
                            warn!("A player joined a server they were already in");
                        }
                        let owner = owner.clone();
                        server_by_user.insert(owner.clone(), world_index);
                        let spawn_centre_chnks_lack = (
                            (player_info.1.x / 32.0).round() as isize,
                            (player_info.1.y / 32.0).round() as isize
                        );
                        let mut constructable_players = vec![];
                        for (us, gp, _) in &saves[world_id].data.players {
                            constructable_players.push((us.clone(), *gp));
                        }
                        let mut new_objs = vec![];
                        run_matrix_nxn(-1..1, |x, y| {
                            new_objs.append(&mut saves[world_index].data.try_generating_objects(
                                (spawn_centre_chnks_lack.0 + x, spawn_centre_chnks_lack.1 + y)
                            ));
                        });
                        let mut all_players = vec![];
                        for object in new_objs {
                            for (user, _, _) in saves[world_id].data.players.clone() {
                                let ip = ip_by_user.get(&user).expect("A user online on a server had no IP address");
                                // if this isn't the player joining...
                                if ip != &from {
                                    // send over the objects
                                    all_players.push((Packet::CreateObject(object.clone()), *ip));
                                }
                            }
                        }
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::JoinedGame(player_info.1, saves[world_id].owner == owner), from));
                        func_send.push((Packet::InventoryState(player_info.2.inventory), from));
                        func_send.push((Packet::OnlinePlayers(constructable_players), from));
                        func_send.push((Packet::AllObjects(saves[world_index].data.objects.clone()), from));
                        func_send.append(&mut other_players);
                        func_send.append(&mut all_players);
                        drop(func_send);
                    }
                    Packet::RequestChunk(chunk) => {
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::RequestChunk");
                        let server = server_by_user.get(owner).expect("Owner is not in a server for Packet::RequestChunk");

                        let chunk_data = saves[*server].data.get_or_gen(chunk);

                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::ChunkData(chunk, chunk_data), from));
                        drop(func_send);
                    }
                    Packet::SendChatMessage(msg) => {
                        // find assoc user
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::RequestMove(GamePosition)");
                        
                        let server = server_by_user.get(owner).expect("Owner is not in a server for Packet::RequestMove(GamePosition)");

                        let mut sendable_message = msg.clone();
                        sendable_message.text.insert_str(0, &format!("[{}] ", owner.username));
                        for player in &saves[*server].data.players {
                            let this_ip = ip_by_user.get(&player.0).expect("Online player has no IP for a requested move");
                            // send message
                            let mut func_send = send.lock().unwrap();
                            func_send.push((Packet::ChatMessage(sendable_message.clone()), *this_ip));
                            drop(func_send);
                        }
                    }
                    Packet::AvalableServers => {
                        // find assoc user
                        let user = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::AvalableServers");
                        let mut profile = None;
                        for tprofile in &profiles {
                            if &tprofile.user == user {
                                profile = Some(tprofile.clone());
                            }
                        }
                        let profile = profile.unwrap();
                        // get servers
                        let mut listings = vec![];
                        for server_id in profile.avalable_games {
                            let this_server = &saves[server_id];
                            listings.push(
                                GameListing {
                                    public_name: this_server.public_name.clone(),
                                    description: String::from("TODO"),
                                    internal_id: server_id,
                                    local: false,
                                    address: String::from("NA/TODO"),
                                    password: false,
                                    played: this_server.played_before.contains(user)
                                }
                            )
                        }
                        // send list
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::ServerList(listings), from));
                        drop(func_send);
                    }
                    Packet::RequestMove(pos) => {
                        // TODO: buffer moves every 10ms to save net space
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::RequestMove(GamePosition)");
                        
                        let server = server_by_user.get(owner).expect("Owner is not in a server for Packet::RequestMove(GamePosition)");
                        
                        let mut self_index = None;

                        for (index, player) in saves[*server].data.players.iter().enumerate() {
                            let this_ip = ip_by_user.get(&player.0).expect("Online player has no IP for a requested move");
                            // send data
                            if this_ip == &from {
                                // but not to the mover
                                self_index = Some(index);
                                continue;
                            }
                            let mut func_send = send.lock().unwrap();
                            func_send.push((Packet::PlayerPositionUpdate(owner.clone(), pos), *this_ip));
                            drop(func_send);
                        }
                        // save data to server
                        saves[*server].data.players[self_index.expect("Owner does not have a datablock in a server.")].1 = pos;
                    }
                    Packet::LeaveWorld => {
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::LeaveWorld");
                        
                        let server = server_by_user.get(owner).expect("Owner is not in a server for Packet::LeaveWorld");
                        
                        let mut self_index = None;

                        for (index, player) in saves[*server].data.players.iter().enumerate() {
                            let this_ip = ip_by_user.get(&player.0).expect("Online player has no IP for a requested disconnect");
                            // send data
                            if this_ip == &from {
                                // but not to the disconnector
                                self_index = Some(index);
                                continue;
                            }
                            let mut func_send = send.lock().unwrap();
                            func_send.push((Packet::PlayerDisconnected(owner.clone()), *this_ip));
                            drop(func_send);
                        }
                        // save disconnect to server
                        let p = saves[*server].data.players.swap_remove(self_index.expect("Owner does not have a datablock in a server."));
                        saves[*server].data.offline_players.push(p);
                    }
                    Packet::WhitelistUser(user) => {
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::WhitelistUser");

                        let server = server_by_user.get(owner).expect("User is not in a server for Packet::WhitelistUser");
                        if &saves[*server].owner == owner {
                            let mut loc = None;
                            for (ind, prof) in profiles.iter().enumerate() {
                                if prof.user == user {
                                    loc = Some(ind);
                                }
                            }
                            if let Some(indexable) = loc {
                                profiles[indexable].avalable_games.push(*server);
                                let mut func_send = send.lock().unwrap();
                                func_send.push((Packet::Whitelisted, from));
                                drop(func_send);
                            }
                            else {
                                let mut func_send = send.lock().unwrap();
                                func_send.push((Packet::UnwhitelistableUser, from));
                                drop(func_send);
                            }
                        }
                        else {
                            let mut func_send = send.lock().unwrap();
                            func_send.push((Packet::NoWhitelistPermission, from));
                            drop(func_send);
                        }
                    }
                    Packet::ActionAnimation(action) => {
                        // find assoc user
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::ActionAnimation");
                        
                        let server = server_by_user.get(owner).expect("Owner is not in a server for Packet::ActionAnimation");

                        // for each player
                        for player in &saves[*server].data.players {
                            let this_ip = ip_by_user.get(&player.0).expect("Online player has no IP for a requested animation");
                            // if this isn't the player who sent originally
                            if this_ip != &from {
                                // send animation
                                let mut func_send = send.lock().unwrap();
                                func_send.push((Packet::ActionAnimation(action), *this_ip));
                                drop(func_send);
                            }
                        }
                    }
                    Packet::RemoveObject(uuid) => {
                        // find assoc user
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::RemoveObject");
                        
                        let server = server_by_user.get(owner).expect("Owner is not in a server for Packet::RemoveObject");

                        // for each player
                        for player in &saves[*server].data.players {
                            let this_ip = ip_by_user.get(&player.0).expect("Online player has no IP for a requested animation");
                            // if this isn't the player who sent originally
                            if this_ip != &from {
                                // reflect removal
                                let mut func_send = send.lock().unwrap();
                                func_send.push((Packet::RemoveObject(uuid), *this_ip));
                                drop(func_send);
                            }
                        }

                        let mut object_index = None;
                        // find given object on server
                        for (index, object) in saves[*server].data.objects.iter().enumerate() {
                            if object.uuid == uuid {
                                object_index = Some(index);
                            }
                        }
                        if object_index.is_none() {
                            warn!("All objects: {:#?}", saves[*server].data.objects);
                            warn!("Requested UUID: {:?}", uuid);
                        }
                        // remove object from server
                        saves[*server].data.objects.remove(object_index.expect("No object found with given uuid for Packet::RemoveObject"));
                    }
                    Packet::UpdateObject(obj) => {
                        // find assoc user
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::UpdateObject");
                        
                        let server = server_by_user.get(owner).expect("Owner is not in a server for Packet::UpdateObject");

                        // for each player
                        for player in &saves[*server].data.players {
                            let this_ip = ip_by_user.get(&player.0).expect("Online player has no IP for a requested update");
                            // if this isn't the player who sent originally
                            if this_ip != &from {
                                // reflect update
                                let mut func_send = send.lock().unwrap();
                                func_send.push((Packet::UpdateObject(obj.clone()), *this_ip));
                                drop(func_send);
                            }
                        }

                        // update object on the server
                        let mut object_index = None;
                        for (index, object) in saves[*server].data.objects.iter().enumerate() {
                            if object.uuid == obj.uuid {
                                object_index = Some(index);
                                break;
                            }
                        }
                        let object_index = object_index.expect("No object found with given uuid for Packet::UpdateObject");
                        saves[*server].data.objects[object_index] = obj;
                    }
    */
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
