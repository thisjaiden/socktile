use std::sync::{Arc, Mutex};

use crate::shared::{listing::GameListing, netty::{NETTY_VERSION, Packet, initiate_host}, player::Player, saves::{Profile, SaveGame, User, profiles, save, save_folder, save_profile, saves}, world::{SPAWN_POSITION, World}};

pub const HOST_PORT: &str = "11111";

pub fn startup() -> ! {
    println!("STARTING GLOBAL GAME SERVER WITH PORT {}. PLEASE CLOSE ALL OTHER INSTANCES OF THE GGS AND APPLICATIONS USING THIS PORT.", HOST_PORT);
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
                    Packet::NettyVersion(version) => {
                        if version == NETTY_VERSION {
                            let mut func_send = send.lock().unwrap();
                            func_send.push((Packet::SameVersion, from));
                            drop(func_send);
                        }
                        else {
                            let mut func_send = send.lock().unwrap();
                            func_send.push((Packet::DifferentVerison, from));
                            drop(func_send);
                        }
                    }
                    Packet::CreateProfile(user) => {
                        let mut tag = 0;
                        for profile in profiles.clone() {
                            if profile.user.username == user.username {
                                if profile.user.tag > tag {
                                    tag = profile.user.tag;
                                }
                            }
                        }
                        let new_profile = Profile {
                            user: User {
                                username: user.username,
                                tag: tag + 1
                            },
                            joined_games: vec![],
                            invited_games: vec![],
                            created_games: vec![]
                        };
                        profiles.push(new_profile.clone());
                        save_profile(new_profile.clone());
                        let mut func_send = send.lock().unwrap();
                        ip_accociations.push((from, new_profile.clone().user));
                        func_send.push((Packet::CreatedProfile(new_profile), from));
                        drop(func_send);
                    }
                    Packet::CreateWorld(name) => {
                        let world_id = saves.last().unwrap().internal_id + 1;
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
                                profiles[index].created_games.push(
                                    GameListing {
                                        public_name: name.clone(),
                                        description: String::new(),
                                        internal_id: world_id,
                                        local: false,
                                        address: String::from("ggs"),
                                        password: false,
                                        played: false
                                    }
                                );
                            }
                        }
                        if owner.tag == 0 {
                            // TODO: Properly
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
                    Packet::JoinWorld(world_uid, user) => {
                        // grab world
                        let world = &mut saves[world_uid].data;
                        // find any existing player and make online if exists
                        let mut existing_player = false;
                        for (index, player) in world.offline_players.clone().into_iter().enumerate() {
                            if player.user == user {
                                let pulled = world.offline_players.remove(index);
                                world.players.push(pulled);
                                existing_player = true;
                                break;
                            }
                        }
                        if !existing_player {
                            world.players.push(Player { user, location: SPAWN_POSITION });
                        }
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::FullWorldData(world.clone()), from));
                        drop(func_send);
                    }
                    Packet::RequestProfile(user) => {
                        let mut found_profile = false;
                        for profile in profiles.clone() {
                            if profile.user.username == user.username {
                                if profile.user.tag == user.tag {
                                    let mut func_send = send.lock().unwrap();
                                    ip_accociations.push((from, user.clone()));
                                    func_send.push((Packet::GiveProfile(profile), from));
                                    drop(func_send);
                                    found_profile = true;
                                }
                            }
                        }
                        if !found_profile {
                            let mut func_send = send.lock().unwrap();
                            func_send.push((Packet::NoProfile, from));
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
