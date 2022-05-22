use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{
        Arc,
        Mutex
    }
};
use bevy::utils::HashMap;
use crate::{
    components::GamePosition,
    consts::{
        NETTY_VERSION,
        NETTY_PORT, TICK_TIME, SAVE_TIME
    },
    shared::{
        netty::Packet,
        saves::User,
        world::World,
        listing::GameListing,
        player::PlayerData
    }
};
use serde::{
    Serialize,
    Deserialize
};

mod tick;

/// Starts the game server!
pub fn startup(arguments: Vec<String>) -> ! {
    // Create our shared packet buffers
    let recv = Arc::new(Mutex::new(vec![]));
    let send = Arc::new(Mutex::new(vec![]));
    // Create a clone of these buffers to move into another thread
    let recv_clone = recv.clone();
    let send_clone = send.clone();
    std::thread::spawn(move || {
        // Start the actual network watching and communication part of the server
        initiate_host(recv_clone, send_clone, arguments);
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
        if timer.elapsed() > std::time::Duration::from_millis(TICK_TIME) {
            // Save every 30 mins
            if autosave.elapsed() > std::time::Duration::from_secs(60 * SAVE_TIME) {
                println!("Saving...");
                for world in saves.clone() {
                    save(world);
                }
                for profile in profiles.clone() {
                    save_profile(profile);
                }
                autosave = std::time::Instant::now();
            }
            // logic tick
            let mut func_send = send.lock().unwrap();
            func_send.append(&mut tick::tick(&mut saves, &ip_by_user));
            drop(func_send);

            // packet instant response and incoming handler
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
                                data: World::new(),
                                path,
                                whitelist: vec![owner.clone()],
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
                            println!("Warning: a player joined a server they were already in");
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
                        let mut new_objs = saves[world_index].data.try_generating_objects(spawn_centre_chnks_lack);
                        new_objs.append(&mut saves[world_index].data.try_generating_objects((spawn_centre_chnks_lack.0, spawn_centre_chnks_lack.1 + 1)));
                        new_objs.append(&mut saves[world_index].data.try_generating_objects((spawn_centre_chnks_lack.0, spawn_centre_chnks_lack.1 - 1)));
                        new_objs.append(&mut saves[world_index].data.try_generating_objects((spawn_centre_chnks_lack.0 + 1, spawn_centre_chnks_lack.1 + 1)));
                        new_objs.append(&mut saves[world_index].data.try_generating_objects((spawn_centre_chnks_lack.0 + 1, spawn_centre_chnks_lack.1 - 1)));
                        new_objs.append(&mut saves[world_index].data.try_generating_objects((spawn_centre_chnks_lack.0 + 1, spawn_centre_chnks_lack.1)));
                        new_objs.append(&mut saves[world_index].data.try_generating_objects((spawn_centre_chnks_lack.0 - 1, spawn_centre_chnks_lack.1 + 1)));
                        new_objs.append(&mut saves[world_index].data.try_generating_objects((spawn_centre_chnks_lack.0 - 1, spawn_centre_chnks_lack.1 - 1)));
                        new_objs.append(&mut saves[world_index].data.try_generating_objects((spawn_centre_chnks_lack.0 - 1, spawn_centre_chnks_lack.1)));
                        let mut all_players = vec![];
                        for object in new_objs {
                            for (user, _, _) in saves[world_id].data.players.clone() {
                                let ip = ip_by_user.get(&user).expect("A user online on a server had no IP address");
                                all_players.push((Packet::CreateObject(object.clone()), *ip));
                            }
                        }
                        let mut func_send = send.lock().unwrap();
                        func_send.push((Packet::JoinedGame(player_info.1, saves[world_id].owner == owner), from));
                        func_send.push((Packet::InventoryState(player_info.2.inventory), from));
                        func_send.push((Packet::OnlinePlayers(constructable_players), from));
                        func_send.push((Packet::AllObjects(saves[world_index].data.objects.clone()), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk(spawn_centre_chnks_lack)), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk((spawn_centre_chnks_lack.0, spawn_centre_chnks_lack.1 + 1))), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk((spawn_centre_chnks_lack.0, spawn_centre_chnks_lack.1 - 1))), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk((spawn_centre_chnks_lack.0 + 1, spawn_centre_chnks_lack.1 + 1))), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk((spawn_centre_chnks_lack.0 + 1, spawn_centre_chnks_lack.1 - 1))), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk((spawn_centre_chnks_lack.0 + 1, spawn_centre_chnks_lack.1))), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk((spawn_centre_chnks_lack.0 - 1, spawn_centre_chnks_lack.1 + 1))), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk((spawn_centre_chnks_lack.0 - 1, spawn_centre_chnks_lack.1 - 1))), from));
                        func_send.push((Packet::ChangesChunk(spawn_centre_chnks_lack, saves[world_index].data.clone_chunk((spawn_centre_chnks_lack.0 - 1, spawn_centre_chnks_lack.1))), from));
                        func_send.append(&mut other_players);
                        func_send.append(&mut all_players);
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
                    _ => {
                        // Ignore this packet, we don't handle it.
                    }
                }
            }
            timer = std::time::Instant::now();
        }
        else {
            std::thread::sleep(std::time::Duration::from_millis(TICK_TIME) - timer.elapsed());
        }
    }
}


/// Returns a `PathBuf` to the folder used for storing profiles.
pub fn profile_folder() -> PathBuf {
    let mut dir = std::env::current_dir().expect("Unable to access the current directory.");
    dir.push("users");
    std::fs::create_dir_all(dir.clone()).expect("Unable to create required directories.");
    dir
}

/// Saves a `Profile` to the disk.
pub fn save_profile(profile: Profile) {
    // Encode profile
    let enc = bincode::serialize(&profile).expect("Unable to serialize a Profile.");

    // Get appropriate path and name
    let mut path = profile_folder();
    path.push(format!("{}{}.bic", profile.user.username, profile.user.tag));

    // Save to disk
    std::fs::write(path, enc).expect("Unable to write a profile to the disk.");
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

pub fn save(save: SaveGame) {
    let enc = bincode::serialize(&save).expect("Unable to serialize a SaveGame.");
    std::fs::write(save.path, enc).expect("Unable to write a SaveGame to disk.");
}

pub fn save_folder() -> PathBuf {
    let mut dir = std::env::current_dir().expect("Unable to access the current directory.");
    dir.push("saves");
    std::fs::create_dir_all(dir.clone()).expect("Unable to create required directories.");
    dir
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SaveGame {
    pub public_name: String,
    pub internal_id: usize,
    pub data: World,
    pub path: PathBuf,
    pub whitelist: Vec<User>,
    pub played_before: Vec<User>,
    pub owner: User,
}

pub fn initiate_host(recv_buffer: Arc<Mutex<Vec<(Packet, SocketAddr)>>>, send_buffer: Arc<Mutex<Vec<(Packet, SocketAddr)>>>, arguments: Vec<String>) -> ! {
    println!("Preparing network functions.");
    println!("Netty version: {}", NETTY_VERSION);
    let mut net = None;
    for (index, argument) in arguments.iter().enumerate() {
        if argument == "port" {
            if arguments.len() > index + 1 {
                println!("Using port {} (overridden)", arguments[index + 1]);
                net = Some(std::net::TcpListener::bind(format!("0.0.0.0:{}", arguments[index + 1])));
            }
            else {
                println!("Invalid argument for port. (none)");
            }
        }
    }
    if net.is_none() {
        println!("Using port {NETTY_PORT} (default)");
        net = Some(std::net::TcpListener::bind(format!("0.0.0.0:{}", NETTY_PORT)));
    }
    
    if let Some(Ok(network)) = net {
        for connection in network.incoming() {
            if let Ok(mut stream) = connection {
                let recv_clone = recv_buffer.clone();
                let send_clone = send_buffer.clone();
                let remote_addr = stream.peer_addr().expect("Unable to get the remote address of a client.");
                let mut stream_clone = stream.try_clone().unwrap();
                std::thread::spawn(move || {
                    let recv = recv_clone;
                    loop {
                        let pkt = Packet::from_read(&mut stream);
                        let mut recv_access = recv.lock().unwrap();
                        if pkt == Packet::FailedDeserialize {
                            // TODO: Signal to disconnect from any services
                            break;
                        }
                        recv_access.push((pkt, remote_addr));
                        drop(recv_access);
                    }
                });
                std::thread::spawn(move || {
                    let send = send_clone;
                    loop {
                        let mut destroy_conenction = false;
                        let mut send_access = send.lock().unwrap();
                        let mut removed = 0;
                        for (index, (packet, address)) in send_access.clone().iter().enumerate() {
                            if packet == &Packet::FailedDeserialize {
                                destroy_conenction = true;
                            }
                            if address == &remote_addr {
                                Packet::to_write(&mut stream_clone, packet.clone());
                                send_access.remove(index - removed);
                                removed += 1;
                            }
                        }
                        drop(send_access);
                        if destroy_conenction {
                            // println!("Dropping connection to {:?}", remote_addr);
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(20));
                    }
                });
            }
            else {
                println!("Warning: Error occured connecting a stream.");
            }
        }
    }
    else {
        panic!("Unable to bind to network effectively. Check that nothing else is running on the same port.");
    }
    unreachable!();
}
