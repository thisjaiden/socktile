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
                drop(globals);
                outgoing.push((Packet::CreatedUser(new_user), source_addr));
            }
        }
        Packet::UserPresence(user) => {
            if user.tag > 0 {
                let mut globals = globals.lock().unwrap();
                globals.user_to_addr.insert(user.clone(), source_addr);
                globals.addr_to_user.insert(source_addr, user);
                drop(globals);
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
            drop(globals);
        }
        Packet::JoinWorld(world_id) => {
            let mut globals = globals.lock().unwrap();
            let packet_user = globals.addr_to_user.get(&source_addr).expect("A user attempted to join a world before first announcing identity.").clone();
            
            let mut world_index = 0;
            for (index, world) in globals.worlds.iter().enumerate() {
                if world.internal_id == world_id {
                    world_index = index;
                    break;
                }
            }
            let mut player_info = None;
            for (index, player) in globals.worlds[world_index].data.offline_players.clone().into_iter().enumerate() {
                if player.0 == packet_user {
                    player_info = Some(globals.worlds[world_index].data.offline_players.remove(index));
                    break;
                }
            }
            if player_info == None {
                player_info = Some((packet_user.clone(), GamePosition { x: 0.0, y: 0.0 }, PlayerData::new()));
            }
            let player_info = player_info.unwrap();
            let mut other_players = vec![];
            for (user, _, _) in globals.worlds[world_id].data.players.clone() {
                let ip = globals.user_to_addr.get(&user).expect("A user online on a server had no IP address");
                other_players.push((Packet::PlayerConnected(packet_user.clone(), player_info.1), *ip));
            }
            if !globals.worlds[world_index].data.players.contains(&player_info) {
                globals.worlds[world_index].data.players.push(player_info.clone());
            }
            else {
                warn!("A player joined a server they were already in");
            }
            let owner = packet_user.clone();
            globals.user_to_world.insert(packet_user.clone(), world_index);
            let spawn_centre_chnks_lack = (
                (player_info.1.x / 32.0).round() as isize,
                (player_info.1.y / 32.0).round() as isize
            );
            let mut constructable_players = vec![];
            for (us, gp, _) in &globals.worlds[world_id].data.players {
                constructable_players.push((us.clone(), *gp));
            }
            let mut new_objs = vec![];
            run_matrix_nxn(-1..1, |x, y| {
                new_objs.append(&mut globals.worlds[world_index].data.try_generating_objects(
                    (spawn_centre_chnks_lack.0 + x, spawn_centre_chnks_lack.1 + y)
                ));
            });
            let mut all_players = vec![];
            for object in new_objs {
                for (user, _, _) in globals.worlds[world_id].data.players.clone() {
                    let ip = globals.user_to_addr.get(&user).expect("A user online on a server had no IP address");
                    // if this isn't the player joining...
                    if ip != &source_addr {
                        // send over the objects
                        all_players.push((Packet::CreateObject(object.clone()), *ip));
                    }
                }
            }
            
            outgoing.push((Packet::JoinedGame(player_info.1, globals.worlds[world_id].owner == owner), source_addr));
            outgoing.push((Packet::AllObjects(globals.worlds[world_index].data.objects.clone()), source_addr));
            drop(globals);
            outgoing.push((Packet::InventoryState(player_info.2.inventory), source_addr));
            outgoing.push((Packet::OnlinePlayers(constructable_players), source_addr));
            outgoing.append(&mut other_players);
            outgoing.append(&mut all_players);
        }
        Packet::RequestChunk(chunk) => {
            let mut globals = globals.lock().unwrap();
            let owner = globals.addr_to_user.get(&source_addr).expect("No user found for an IP adress used with Packet::RequestChunk");
            let server = globals.user_to_world.get(owner).expect("Owner is not in a server for Packet::RequestChunk").clone();

            let chunk_data = globals.worlds[server].data.get_or_gen(chunk);
            drop(globals);

            outgoing.push((Packet::ChunkData(chunk, chunk_data), source_addr));
        }
        Packet::RequestMove(pos) => {
            let mut globals = globals.lock().unwrap();
            let owner = globals.addr_to_user.get(&source_addr).expect("No user found for an IP adress used with Packet::RequestMove(GamePosition)").clone();
            
            let server = globals.user_to_world.get(&owner).expect("Owner is not in a server for Packet::RequestMove(GamePosition)").clone();
            
            let mut self_index = None;

            for (index, player) in globals.worlds[server].data.players.iter().enumerate() {
                let this_ip = globals.user_to_addr.get(&player.0).expect("Online player has no IP for a requested move");
                // send data
                if this_ip == &source_addr {
                    // but not to the mover
                    self_index = Some(index);
                    continue;
                }
                outgoing.push((Packet::PlayerPositionUpdate(owner.clone(), pos), *this_ip));
            }
            // save data to server
            globals.worlds[server].data.players[self_index.expect("Owner does not have a datablock in a server.")].1 = pos;
            drop(globals);
        }
        _ => todo!()
    }
    outgoing
}
