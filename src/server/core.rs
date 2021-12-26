use std::sync::{Arc, Mutex};

use crate::{components::GamePosition, server::saves::{Profile, SaveGame, profiles, save, save_folder, save_profile, saves}, shared::{netty::{NETTY_VERSION, Packet, initiate_host}, player::Player, saves::User, world::World}};

pub const HOST_PORT: &str = "11111";

pub fn startup() -> ! {
    println!("GGS using port *:{}", HOST_PORT);
    let recv = Arc::new(Mutex::new(vec![]));
    let send = Arc::new(Mutex::new(vec![]));
    let recv_clone = recv.clone();
    let send_clone = send.clone();
    std::thread::spawn(move || {
        initiate_host(recv_clone, send_clone);
    });
    let mut timer = std::time::Instant::now();
    let mut autosave = std::time::Instant::now();
    let mut saves = saves();
    let mut profiles = profiles();
    let mut ip_accociations = vec![];
    let mut sorted = vec![];
    println!("Sorting saves...");
    for i in 0..saves.len() {
        for save in saves.clone() {
            if save.internal_id == i {
                sorted.push(save);
            }
        }
    }
    saves = sorted;
    println!("Saves sorted. Server started!");
    loop {
        if timer.elapsed() > std::time::Duration::from_millis(50) {
            // autosave every 30 mins (IMPORTANT: change this to 5 on production)
            if autosave.elapsed() > std::time::Duration::from_secs(60 * 30) {
                println!("Autosaving...");
                for world in saves.clone() {
                    save(world);
                }
                autosave = std::time::Instant::now();
            }
            // rest of tick
            let mut func_recv = recv.lock().unwrap();
            let incoming_data = func_recv.clone();
            func_recv.clear();
            drop(func_recv);
            for (packet, from) in incoming_data {
                match packet {
                    Packet::NettyVersion(_) => {
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::NettyStable, from));
                        drop(func_send);
                    }
                    Packet::CreateUser(user) => {
                        let mut tag = 0;
                        for profile in profiles.clone() {
                            if profile.user.username == user.username && profile.user.tag > tag {
                                tag = profile.user.tag;
                            }
                        }
                        let new_user = User {
                            username: user.username,
                            tag: tag + 1
                        };
                        let new_profile = Profile {
                            user: new_user.clone(),
                            avalable_games: vec![]
                        };
                        profiles.push(new_profile.clone());
                        save_profile(new_profile.clone());
                        let mut func_send = send.lock().unwrap();
                        ip_accociations.push((from, new_user.clone()));
                        func_send.push((Packet::CreatedUser(new_user), from));
                        drop(func_send);
                    }
                    Packet::UserPresence(user) => {
                        if user.tag > 0 {
                            ip_accociations.push((from, user));
                        }
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::NettyStable, from));
                        drop(func_send);
                    }
                    Packet::CreateWorld(name) => {
                        let mut world_id = 0;
                        if let Some(last) = saves.last() {
                            world_id = last.internal_id + 1;
                        }
                        let mut path = save_folder();
                        path.push(format!("world_{}.bic", world_id));
                        let mut owner = User {
                            username: String::new(),
                            tag: 0
                        };
                        for (address_pair, user_pair) in ip_accociations.clone() {
                            if address_pair == from {
                                owner = user_pair;
                            }
                        }
                        for (index, profile) in profiles.clone().into_iter().enumerate() {
                            if owner == profile.user {
                                profiles[index].avalable_games.push(world_id);
                            }
                        }
                        if owner.tag == 0 {
                            // TODO: Properly handle
                            panic!("No user found for an IP address used with Packet::CreateWorld(String)");
                        }
                        saves.push(
                            SaveGame {
                                public_name: name,
                                internal_id: world_id,
                                version: String::from(NETTY_VERSION),
                                data: World::new(),
                                path,
                                whitelist: None,
                                blacklist: vec![],
                                played_before: vec![],
                                owner
                            }
                        );
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::CreatedWorld(saves.last().unwrap().internal_id), from));
                        drop(func_send);
                    }
                    Packet::JoinWorld(world_id) => {
                        let mut owner = User {
                            username: String::new(),
                            tag: 0
                        };
                        for (address_pair, user_pair) in ip_accociations.clone() {
                            if address_pair == from {
                                owner = user_pair;
                            }
                        }
                        for (index, profile) in profiles.clone().into_iter().enumerate() {
                            if owner == profile.user {
                                profiles[index].avalable_games.push(world_id);
                            }
                        }
                        if owner.tag == 0 {
                            // TODO: Properly handle
                            panic!("No user found for an IP address used with Packet::JoinWorld(usize)");
                        }
                        let mut world_index = 0;
                        for (index, world) in saves.iter().enumerate() {
                            if world.internal_id == world_id {
                                world_index = index;
                                break;
                            }
                        }
                        let mut has_joined = false;
                        let mut player_info = None;
                        for (index, player) in saves[world_index].data.offline_players.clone().into_iter().enumerate() {
                            if player.user == owner {
                                has_joined = true;
                                player_info = Some(saves[world_index].data.offline_players.remove(index));
                                break;
                            }
                        }
                        if !has_joined {
                            player_info = Some(Player {
                                user: owner,
                                location: GamePosition { x: 0.0, y: 0.0 }
                            });
                        }
                        let spawn_centre_chnks_lack = (
                            (player_info.clone().unwrap().location.x / 32.0).round() as usize,
                            (player_info.clone().unwrap().location.y / 32.0).round() as usize
                        );
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::JoinedGame(player_info.clone().unwrap().location), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk(spawn_centre_chnks_lack)), from));
                        drop(func_send);
                    }
                    _ => {
                        // Ignore this packet, we don't handle it.
                    }
                }
            }
            timer = std::time::Instant::now();
        }
        else {
            std::thread::sleep(std::time::Duration::from_millis(50) - timer.elapsed());
        }
    }
}
