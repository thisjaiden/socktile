use crate::{prelude::*, shared::listing::GameListing};

use super::{tick::save_folder, world, Profile, SaveGame};

pub fn handler(
    packet: Packet,
    globals: std::sync::Arc<std::sync::Mutex<Globals>>,
    source_addr: std::net::SocketAddr,
) -> Vec<(Packet, std::net::SocketAddr)> {
    let mut outgoing = vec![];
    match packet {
        Packet::NettyVersion(v) => {
            if v == NETTY_VERSION {
                outgoing.push((Packet::AllSet, source_addr));
            }
            else {
                outgoing.push((
                    Packet::WrongVersion(String::from(NETTY_VERSION)),
                    source_addr,
                ));
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
                tag: tag + 1,
            };
            if new_user.tag > 9999 {
                outgoing.push((Packet::OverusedName, source_addr));
            }
            else {
                let new_profile = Profile {
                    user: new_user.clone(),
                    avalable_games: vec![],
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
                .replace([
                        '\\', '/', ':', '*', '?',
                        '"', '<', '>', '|', '.',
                        '\'',
                    ],
                    "I"
                );
            // Don't allow world names to be longer than 10 characters
            if rname.chars().count() > 10 {
                // dirty code to grab the first 10 characters
                rname = rname
                    .chars()
                    .collect::<Vec<char>>()
                    .split_at(10)
                    .0
                    .iter()
                    .collect();
            }

            path.push(format!("{}_{}.bic", rname, world_id));
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect("No user found for an IP adress used with Packet::CreateWorld(String)")
                .clone();
            for (index, profile) in globals.profiles.clone().into_iter().enumerate() {
                if owner == profile.user {
                    globals.profiles[index].avalable_games.push(world_id);
                }
            }
            let owner = owner.clone();
            globals.worlds.push(SaveGame {
                public_name: name,
                internal_id: world_id,
                data: world::World::new(),
                path,
                whitelist: vec![owner.clone()],
                played_before: vec![],
                owner,
            });
            outgoing.push((
                Packet::CreatedWorld(globals.worlds.last().unwrap().internal_id),
                source_addr,
            ));
            drop(globals);
        }
        Packet::JoinWorld(world_id) => {
            let mut globals = globals.lock().unwrap();
            let packet_user = globals
                .addr_to_user
                .get(&source_addr)
                .expect("A user attempted to join a world before first announcing identity.")
                .clone();

            let mut world_index = 0;
            for (index, world) in globals.worlds.iter().enumerate() {
                if world.internal_id == world_id {
                    world_index = index;
                    break;
                }
            }
            let mut player_info = None;
            for (index, player) in globals.worlds[world_index]
                .data
                .offline_players
                .clone()
                .into_iter()
                .enumerate()
            {
                if player.0 == packet_user {
                    player_info = Some(
                        globals.worlds[world_index]
                            .data
                            .offline_players
                            .remove(index),
                    );
                    break;
                }
            }
            if player_info.is_none() {
                player_info = Some((
                    packet_user.clone(),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    PlayerData::new(),
                ));
            }
            let player_info = player_info.unwrap();
            let mut other_players = vec![];
            for (user, _, _) in globals.worlds[world_id].data.players.clone() {
                let ip = globals
                    .user_to_addr
                    .get(&user)
                    .expect("A user online on a server had no IP address");
                other_players.push((
                    Packet::PlayerConnected(packet_user.clone(), player_info.1),
                    *ip,
                ));
            }
            if !globals.worlds[world_index]
                .data
                .players
                .contains(&player_info)
            {
                globals.worlds[world_index]
                    .data
                    .players
                    .push(player_info.clone());
            }
            else {
                warn!("A player joined a server they were already in");
            }
            let owner = packet_user.clone();
            globals
                .user_to_world
                .insert(packet_user.clone(), world_index);
            let spawn_centre_chnks_lack = (
                (player_info.1.translation.x / 32.0).round() as isize,
                (player_info.1.translation.y / 32.0).round() as isize,
            );
            let mut constructable_players = vec![];
            for (us, gp, _) in &globals.worlds[world_id].data.players {
                constructable_players.push((us.clone(), *gp));
            }
            let mut new_objs = vec![];
            run_matrix_nxn(-2..2, |x, y| {
                new_objs.append(
                    &mut globals.worlds[world_index].data.try_generating_objects((
                        spawn_centre_chnks_lack.0 + x,
                        spawn_centre_chnks_lack.1 + y,
                    )),
                );
            });
            let mut all_players = vec![];
            for object in new_objs {
                for (user, _, _) in globals.worlds[world_id].data.players.clone() {
                    let ip = globals
                        .user_to_addr
                        .get(&user)
                        .expect("A user online on a server had no IP address");
                    // if this isn't the player joining...
                    if ip != &source_addr {
                        // send over the objects
                        all_players.push((Packet::CreateObject(object.clone()), *ip));
                    }
                }
            }

            outgoing.push((
                Packet::JoinedGame(player_info.1, globals.worlds[world_id].owner == owner),
                source_addr,
            ));
            outgoing.push((
                Packet::AllObjects(globals.worlds[world_index].data.objects.clone()),
                source_addr,
            ));
            drop(globals);
            outgoing.push((Packet::InventoryState(player_info.2.inventory), source_addr));
            outgoing.push((Packet::OnlinePlayers(constructable_players), source_addr));
            outgoing.append(&mut other_players);
            outgoing.append(&mut all_players);
        }
        Packet::RequestChunk(chunk) => {
            let mut globals = globals.lock().unwrap();
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect("No user found for an IP adress used with Packet::RequestChunk");
            let server = *globals
                .user_to_world
                .get(owner)
                .expect("Owner is not in a server for Packet::RequestChunk");

            let chunk_data = globals.worlds[server].data.get_or_gen(chunk);
            drop(globals);

            outgoing.push((Packet::ChunkData(chunk, chunk_data), source_addr));
        }
        Packet::RequestMove(pos) => {
            let mut globals = globals.lock().unwrap();
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect(
                    "No user found for an IP adress used with Packet::RequestMove(GamePosition)",
                )
                .clone();

            let server = *globals
                .user_to_world
                .get(&owner)
                .expect("Owner is not in a server for Packet::RequestMove(GamePosition)");

            let mut self_index = None;

            for (index, player) in globals.worlds[server].data.players.iter().enumerate() {
                let this_ip = globals
                    .user_to_addr
                    .get(&player.0)
                    .expect("Online player has no IP for a requested move");
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
        Packet::AvalableServers => {
            let globals = globals.lock().unwrap();
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect("No user found for an IP address used with Packet::AvalableServers");

            // find assoc user
            let mut profile = None;
            for tprofile in &globals.profiles {
                if &tprofile.user == owner {
                    profile = Some(tprofile.clone());
                }
            }
            let profile = profile.expect("No profile found for Packet::AvalableServers");
            // get servers
            let mut listings = vec![];
            for server_id in profile.avalable_games {
                let this_server = &globals.worlds[server_id];
                listings.push(GameListing {
                    public_name: this_server.public_name.clone(),
                    description: String::from("TODO"),
                    internal_id: server_id,
                    local: false,
                    address: String::from("NA/TODO"),
                    password: false,
                    played: this_server.played_before.contains(owner),
                })
            }
            drop(globals);
            // send list
            outgoing.push((Packet::ServerList(listings), source_addr));
        }
        Packet::WhitelistUser(user) => {
            let mut globals = globals.lock().unwrap();
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect("No user found for an IP adress used with Packet::WhitelistUser");

            let server = *globals
                .user_to_world
                .get(owner)
                .expect("User is not in a server for Packet::WhitelistUser");
            if &globals.worlds[server].owner == owner {
                let mut loc = None;
                for (ind, prof) in globals.profiles.iter().enumerate() {
                    if prof.user == user {
                        loc = Some(ind);
                    }
                }
                if let Some(indexable) = loc {
                    globals.profiles[indexable].avalable_games.push(server);
                    outgoing.push((Packet::Whitelisted, source_addr));
                }
                else {
                    outgoing.push((Packet::UnwhitelistableUser, source_addr));
                }
            }
            else {
                outgoing.push((Packet::NoWhitelistPermission, source_addr));
            }
            drop(globals);
        }
        Packet::LeaveWorld => {
            let mut globals = globals.lock().unwrap();
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect("No user found for an IP adress used with Packet::LeaveWorld");

            let server = *globals
                .user_to_world
                .get(owner)
                .expect("Owner is not in a server for Packet::LeaveWorld");

            let mut self_index = None;

            for (index, player) in globals.worlds[server].data.players.iter().enumerate() {
                let this_ip = globals
                    .user_to_addr
                    .get(&player.0)
                    .expect("Online player has no IP for a requested disconnect");
                // send data
                if this_ip == &source_addr {
                    // but not to the disconnector
                    self_index = Some(index);
                    continue;
                }
                outgoing.push((Packet::PlayerDisconnected(owner.clone()), *this_ip));
            }
            // save disconnect to server
            let p = globals.worlds[server]
                .data
                .players
                .swap_remove(self_index.expect("Owner does not have a datablock in a server."));
            globals.worlds[server].data.offline_players.push(p);
            drop(globals);
        }
        Packet::SendChatMessage(msg) => {
            let globals = globals.lock().unwrap();
            // find assoc user
            let owner = globals.addr_to_user.get(&source_addr).expect(
                "No user found for an IP adress used with Packet::RequestMove(GamePosition)",
            );

            let server = *globals
                .user_to_world
                .get(owner)
                .expect("Owner is not in a server for Packet::RequestMove(GamePosition)");

            let mut sendable_message = msg.clone();
            sendable_message
                .text
                .insert_str(0, &format!("[{}] ", owner.username));
            for player in &globals.worlds[server].data.players {
                let this_ip = globals
                    .user_to_addr
                    .get(&player.0)
                    .expect("Online player has no IP for a requested move");
                // send message
                outgoing.push((Packet::ChatMessage(sendable_message.clone()), *this_ip));
            }
            drop(globals);
        }
        Packet::UpdateObject(obj) => {
            let mut globals = globals.lock().unwrap();
            // find assoc user
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect("No user found for an IP adress used with Packet::UpdateObject");

            let server = *globals
                .user_to_world
                .get(owner)
                .expect("Owner is not in a server for Packet::UpdateObject");

            // for each player
            for player in &globals.worlds[server].data.players {
                let this_ip = globals
                    .user_to_addr
                    .get(&player.0)
                    .expect("Online player has no IP for a requested update");
                // if this isn't the player who sent originally
                if this_ip != &source_addr {
                    // reflect update
                    outgoing.push((Packet::UpdateObject(obj.clone()), *this_ip));
                }
            }

            // update object on the server
            let mut object_index = None;
            for (index, object) in globals.worlds[server].data.objects.iter().enumerate() {
                if object.uuid == obj.uuid {
                    object_index = Some(index);
                    break;
                }
            }
            let object_index =
                object_index.expect("No object found with given uuid for Packet::UpdateObject");
            globals.worlds[server].data.objects[object_index] = obj;
            drop(globals);
        }
        Packet::RemoveObject(uuid) => {
            let mut globals = globals.lock().unwrap();
            // find assoc user
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect("No user found for an IP adress used with Packet::RemoveObject");

            let server = *globals
                .user_to_world
                .get(owner)
                .expect("Owner is not in a server for Packet::RemoveObject");

            // for each player
            for player in &globals.worlds[server].data.players {
                let this_ip = globals
                    .user_to_addr
                    .get(&player.0)
                    .expect("Online player has no IP for a requested animation");
                // if this isn't the player who sent originally
                if this_ip != &source_addr {
                    // reflect removal
                    outgoing.push((Packet::RemoveObject(uuid), *this_ip));
                }
            }

            let mut object_index = None;
            // find given object on server
            for (index, object) in globals.worlds[server].data.objects.iter().enumerate() {
                if object.uuid == uuid {
                    object_index = Some(index);
                }
            }
            if object_index.is_none() {
                warn!("All objects: {:#?}", globals.worlds[server].data.objects);
                warn!("Requested UUID: {:?}", uuid);
                warn!("Unable to remove this object from a given world. Please Investigate!");
            }
            let object_position = globals.worlds[server].data.objects[object_index.unwrap()]
                .pos;
            let object_representation = globals.worlds[server].data.objects[object_index.unwrap()]
                .rep
                .clone();
            match object_representation {
                ObjectType::Tree(_) => {
                    // spawn 2-3 wood
                    let amount = random(2, 3);
                    for _ in 0..amount {
                        let x_offset = random(0, 64) as f32;
                        let y_offset = random(0, 64) as f32;
                        let uuid = uuid::Uuid::from_u128(rand::random());
                        let n_object = Object {
                            pos: Transform::from_xyz(
                                object_position.translation.x + x_offset - 32.0,
                                object_position.translation.y + y_offset - 32.0,
                                0.0,
                            ),
                            rep: ObjectType::GroundItem(Item::Wood),
                            uuid,
                        };
                        for player in &globals.worlds[server].data.players {
                            let this_ip = globals
                                .user_to_addr
                                .get(&player.0)
                                .expect("Online player has no IP for a requested animation");
                            // send new object packet
                            outgoing.push((Packet::CreateObject(n_object.clone()), *this_ip));
                        }
                        globals.worlds[server].data.objects.push(n_object);
                    }
                }
                _ => {}
            }
            // remove object from server
            globals.worlds[server].data.objects.remove(
                object_index.expect("No object found with given uuid for Packet::RemoveObject"),
            );
            drop(globals);
        }
        Packet::ActionAnimation(action) => {
            let globals = globals.lock().unwrap();
            // find assoc user
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect("No user found for an IP adress used with Packet::ActionAnimation");

            let server = *globals
                .user_to_world
                .get(owner)
                .expect("Owner is not in a server for Packet::ActionAnimation");

            // for each player
            for player in &globals.worlds[server].data.players {
                let this_ip = globals
                    .user_to_addr
                    .get(&player.0)
                    .expect("Online player has no IP for a requested animation");
                // if this isn't the player who sent originally
                if this_ip != &source_addr {
                    // send animation
                    outgoing.push((Packet::ActionAnimation(action), *this_ip));
                }
            }
            drop(globals);
        }
        Packet::TileUpdate(chunk, tile, tilestate) => {
            let mut globals = globals.lock().unwrap();
            // find assoc user
            let owner = globals
                .addr_to_user
                .get(&source_addr)
                .expect("No user found for an IP adress used with Packet::TileUpdate");

            let server = *globals
                .user_to_world
                .get(owner)
                .expect("Owner is not in a server for Packet::TileUpdate");
            // for each player
            for player in &globals.worlds[server].data.players {
                let this_ip = globals
                    .user_to_addr
                    .get(&player.0)
                    .expect("Online player has no IP for a requested animation");
                // if this isn't the player who sent originally
                if this_ip != &source_addr {
                    // reflect
                    outgoing.push((Packet::TileUpdate(chunk, tile, tilestate), *this_ip));
                }
            }
            globals.worlds[server].data.modify_tile(chunk, tile, tilestate);
            drop(globals);
        }
        _ => todo!(),
    }
    outgoing
}
