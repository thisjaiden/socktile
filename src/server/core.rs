use std::{sync::{Arc, Mutex}, net::SocketAddr};

use bevy::utils::HashMap;

use crate::{components::GamePosition, server::saves::{Profile, SaveGame, profiles, save, save_folder, save_profile, saves}, shared::{netty::{NETTY_VERSION, Packet, initiate_host}, player::Player, saves::User, world::World, listing::GameListing}};

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
    let mut ip_by_user: HashMap<User, SocketAddr> = HashMap::default();
    let mut user_by_ip: HashMap<SocketAddr, User> = HashMap::default();
    let mut server_by_user: HashMap<User, usize> = HashMap::default();
    let mut sorted = vec![];
    println!("{} profiles and {} saves.", profiles.len(), saves.len());
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
                for profile in profiles.clone() {
                    save_profile(profile);
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
                    Packet::NettyVersion(v) => {
                        let mut func_send = send.lock().unwrap();
                        if v == NETTY_VERSION {
                            func_send.push((Packet::AllSet, from));
                        }
                        else {
                            func_send.push((Packet::WrongVersion(String::from(NETTY_VERSION)), from));
                        }
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
                        if new_user.tag > 9999 {
                            let mut func_send = send.lock().unwrap();
                            func_send.push((Packet::OverusedName, from));
                            drop(func_send);
                            break;
                        }
                        let new_profile = Profile {
                            user: new_user.clone(),
                            avalable_games: vec![]
                        };
                        profiles.push(new_profile.clone());
                        save_profile(new_profile.clone());
                        let mut func_send = send.lock().unwrap();
                        ip_by_user.insert(new_user.clone(), from);
                        user_by_ip.insert(from, new_user.clone());
                        func_send.push((Packet::CreatedUser(new_user), from));
                        drop(func_send);
                    }
                    Packet::UserPresence(user) => {
                        if user.tag > 0 {
                            ip_by_user.insert(user.clone(), from);
                            user_by_ip.insert(from, user);
                        }
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::AllSet, from));
                        drop(func_send);
                    }
                    Packet::CreateWorld(name) => {
                        let mut world_id = 0;
                        if let Some(last) = saves.last() {
                            world_id = last.internal_id + 1;
                        }
                        let mut path = save_folder();
                        path.push(format!("world_{}.bic", world_id));
                        let owner = user_by_ip.get(&from).expect("No user found for an IP adress used with Packet::CreateWorld(String)");
                        for (index, profile) in profiles.clone().into_iter().enumerate() {
                            if owner == &profile.user {
                                profiles[index].avalable_games.push(world_id);
                            }
                        }
                        let owner = owner.clone();
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
                            if &player.user == owner {
                                player_info = Some(saves[world_index].data.offline_players.remove(index));
                                break;
                            }
                        }
                        if player_info == None {
                            player_info = Some(Player {
                                user: owner.clone(),
                                location: GamePosition { x: 0.0, y: 0.0 }
                            });
                        }
                        saves[world_index].data.players.push(player_info.clone().unwrap());
                        let owner = owner.clone();
                        server_by_user.insert(owner.clone(), world_index);
                        let spawn_centre_chnks_lack = (
                            (player_info.clone().unwrap().location.x / 32.0).round() as isize,
                            (player_info.clone().unwrap().location.y / 32.0).round() as isize
                        );
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::JoinedGame(player_info.clone().unwrap().location, saves[world_id].owner == owner), from));
                        func_send.push((Packet::OnlinePlayers(saves[world_id].data.players.clone()), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk(spawn_centre_chnks_lack)), from));
                        drop(func_send);
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

                        for player in &saves[*server].data.players {
                            let this_ip = ip_by_user.get(&player.user).expect("Online player has no IP for a requested move");
                            // send data
                            if this_ip == &from {
                                // but not to the mover
                                break;
                            }
                            let mut func_send = send.lock().unwrap();
                            func_send.push((Packet::PlayerPositionUpdate(owner.clone(), pos), *this_ip));
                            drop(func_send);
                        }
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
