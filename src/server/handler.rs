use crate::prelude::*;

use super::{Profile, tick::save_folder, SaveGame, world};

pub fn handler(
    packet: Packet,
    globals: std::sync::Arc<std::sync::Mutex<Globals>>,
    source_addr: std::net::SocketAddr
) -> Vec<(Packet, std::net::SocketAddr)> {
    let mut outgoing = vec![];
    match packet {
        Packet::NettyVersion(v) => {
            if v == NETTY_VERSION {
                outgoing.push((Packet::AllSet, source_addr));
            }
            else {
                outgoing.push((Packet::WrongVersion(String::from(NETTY_VERSION)), source_addr));
            }
        }
        Packet::CreateUser(user) => {
            let mut globals = globals.lock().unwrap();
            let mut tag = 0;
            for profile in globals.profiles.clone() {
                if profile.user.username == user.username && profile.user.tag > tag {
                    tag = profile.user.tag;
                }
            }
            let new_user = User {
                username: user.username,
                tag: tag + 1
            };
            if new_user.tag > 9999 {
                outgoing.push((Packet::OverusedName, source_addr));
            }
            else {
                let new_profile = Profile {
                    user: new_user.clone(),
                    avalable_games: vec![]
                };
                globals.profiles.push(new_profile.clone());
                globals.user_to_addr.insert(new_user.clone(), source_addr);
                globals.addr_to_user.insert(source_addr, new_user.clone());
                outgoing.push((Packet::CreatedUser(new_user), source_addr));
            }
        }
        Packet::UserPresence(user) => {
            if user.tag > 0 {
                let mut globals = globals.lock().unwrap();
                globals.user_to_addr.insert(user.clone(), source_addr);
                globals.addr_to_user.insert(source_addr, user);
            }
            outgoing.push((Packet::AllSet, source_addr));
        }
        Packet::CreateWorld(name) => {
            let mut globals = globals.lock().unwrap();
            let mut world_id = 0;
            if let Some(last) = globals.worlds.last() {
                world_id = last.internal_id + 1;
            }
            let mut path = save_folder();

            // This replaces invalid characters (ones that would break file paths) with "I".
            // On windows these are \ / : * ? " < > |
            // I've also included  . and ' just in case
            let mut rname = name.clone();
            rname = rname
                .replace('\\', "I")
                .replace('/', "I")
                .replace(':', "I")
                .replace('*', "I")
                .replace('?', "I")
                .replace('"', "I")
                .replace('<', "I")
                .replace('>', "I")
                .replace('|', "I")
                .replace('.', "I")
                .replace('\'', "I");
            // Don't allow world names to be longer than 10 characters
            if rname.chars().count() > 10 {
                // dirty code to grab the first 10 characters
                rname = rname.chars().collect::<Vec<char>>().split_at(10).0.iter().collect();
            }
            
            path.push(format!("{}_{}.bic", rname, world_id));
            let owner = globals.addr_to_user.get(&source_addr)
                .expect("No user found for an IP adress used with Packet::CreateWorld(String)")
                .clone();
            for (index, profile) in globals.profiles.clone().into_iter().enumerate() {
                if owner == profile.user {
                    globals.profiles[index].avalable_games.push(world_id);
                }
            }
            let owner = owner.clone();
            globals.worlds.push(
                SaveGame {
                    public_name: name,
                    internal_id: world_id,
                    data: world::World::new(),
                    path,
                    whitelist: vec![owner.clone()],
                    played_before: vec![],
                    owner
                }
            );
            outgoing.push((Packet::CreatedWorld(globals.worlds.last().unwrap().internal_id), source_addr));
        }
        _ => todo!()
    }
    outgoing
}
