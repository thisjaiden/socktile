use crate::prelude::*;

use super::Profile;

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
        _ => todo!()
    }
    outgoing
}
