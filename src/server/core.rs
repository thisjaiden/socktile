use std::sync::{Arc, Mutex};

use crate::shared::{netty::{NETTY_VERSION, Packet, initiate_host}, saves::{Profile, SaveGame, User, profiles, save_folder, save_profile, saves}, world::World};

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
    let mut saves = saves();
    let mut profiles = profiles();
    let mut ip_accociations = vec![];
    loop {
        if timer.elapsed() > std::time::Duration::from_millis(50) {
            let mut func_recv = recv.lock().unwrap();
            let incoming_data = func_recv.clone();
            func_recv.clear();
            drop(func_recv);
            for (packet, from) in incoming_data {
                println!("{}: {:?}", from, packet);
                match packet {
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
                        let mut world_id = 0;
                        for save in saves.clone() {
                            if save.internal_id > world_id {
                                world_id = save.internal_id;
                            }
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
                        if owner.tag == 0 {
                            panic!("No user found for an IP adress used with Packet::CreateWorld(String)");
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
                        todo!();
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
                    p => {
                        println!("No handler from packet {:?}", p);
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
