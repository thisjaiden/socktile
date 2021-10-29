use std::sync::{Arc, Mutex};

use crate::shared::{netty::{initiate_host, Packet}, saves::{Profile, User, profiles, save_profile, saves}};

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
                        func_send.push((Packet::CreatedProfile(new_profile), from));
                        drop(func_send);
                    }
                    Packet::RequestProfile(user) => {
                        let mut found_profile = false;
                        for profile in profiles.clone() {
                            if profile.user.username == user.username {
                                if profile.user.tag == user.tag {
                                    let mut func_send = send.lock().unwrap();
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
