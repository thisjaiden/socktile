use crate::{GameState, consts::*, shared::netty::Packet};

use std::{net::TcpStream, sync::{Arc, Mutex}};

use bevy::prelude::*;

use super::{Reality, Disk, chat::ChatMessage};

pub struct Netty {
    connection: ConnectionStatus,
    input: Arc<Mutex<Vec<Packet>>>,
    output: Arc<Mutex<Vec<Packet>>>,
}

impl Netty {
    pub fn init() -> Netty {
        info!("Netty initalizing");

        let connection = TcpStream::connect_timeout(
            &std::net::SocketAddr::from((GGS, NETTY_PORT)),
            std::time::Duration::from_secs(TIMEOUT_DURATION)
        );
        if let Ok(con) = connection {
            let inp = Arc::new(Mutex::new(vec![]));
            let out = Arc::new(Mutex::new(vec![]));
            info!("Good connection to GGS (LOCAL-DEV), starting up client.");
            startup(con, inp.clone(), out.clone());
            let mut fin = Netty {
                connection: ConnectionStatus::Connected,
                input: inp,
                output: out
            };
            fin.say(Packet::NettyVersion(String::from(NETTY_VERSION)));
            return fin;
        }
        else {
            if !google_exists() {
                return Netty {
                    connection: ConnectionStatus::NoInternet,
                    input: Arc::new(Mutex::new(vec![])),
                    output: Arc::new(Mutex::new(vec![]))
                };
            }
            return Netty {
                connection: ConnectionStatus::NoGGS,
                input: Arc::new(Mutex::new(vec![])),
                output: Arc::new(Mutex::new(vec![]))
            }
        }
    }
    pub fn connection(&mut self) -> ConnectionStatus {
        self.connection
    }
    pub fn say(&mut self, packet: Packet) {
        let mut out = self.output.lock().unwrap();
        out.push(packet);
        drop(out);
    }
    pub fn step(&mut self, reality: &mut Reality, disk: &mut Disk) {
        let mut input = self.input.lock().unwrap();
        let pkts = input.clone();
        input.clear();
        drop(input);
        for packet in pkts {
            match packet {
                Packet::CreatedUser(user) => {
                    while !disk.update_user(user.clone()) {};
                    info!("Saved new user information.");
                }
                Packet::AllSet => {
                    self.connection = ConnectionStatus::Stable;
                }
                Packet::CreatedWorld(id) => {
                    self.say(Packet::JoinWorld(id));
                }
                Packet::JoinedGame(mypos, ownership) => {
                    reality.set_player_position(mypos);
                    reality.set_ownership(ownership);
                }
                Packet::ChunkData(chunk, data) => {
                    reality.add_chunk(chunk, data);
                }
                Packet::ServerList(servers) => {
                    reality.set_avalable_servers(servers);
                }
                Packet::WrongVersion(prefered_version) => {
                    error!("Server is running {}, and you're using {} (You're most likely out of date, update!)", prefered_version, NETTY_VERSION);
                    panic!("{FATAL_ERROR}");
                }
                Packet::OnlinePlayers(players) => {
                    reality.add_online_players(players);
                }
                Packet::PlayerConnected(user, pos) => {
                    reality.add_online_players(vec![(user, pos)]);
                }
                Packet::PlayerDisconnected(user) => {
                    reality.disconnect_player(user);
                }
                Packet::PlayerPositionUpdate(p, l) => {
                    reality.queue_player_move(p, l);
                }
                Packet::Whitelisted => {
                    reality.queue_chat(ChatMessage {
                        text: String::from("User added to whitelist!"),
                        color: Color::BLACK,
                        sent_at: std::time::Instant::now()
                    });
                }
                Packet::NoWhitelistPermission => {
                    reality.queue_chat(ChatMessage {
                        text: String::from("You don't have permission to whitelist other users."),
                        color: Color::RED,
                        sent_at: std::time::Instant::now()
                    });
                }
                Packet::UnwhitelistableUser => {
                    reality.queue_chat(ChatMessage {
                        text: String::from("Unable to whitelist user. (Did you spell everything right?)"),
                        color: Color::RED,
                        sent_at: std::time::Instant::now()
                    });
                }
                Packet::InventoryState(inventory) => {
                    reality.set_inventory(inventory);
                }
                Packet::CreateObject(object) => {
                    reality.spawn_object(object);
                }
                Packet::UpdateObject(object) => {
                    reality.update_object(object);
                }
                Packet::RemoveObject(uuid) => {
                    reality.remove_object(uuid);
                }
                Packet::AllObjects(objects) => {
                    for object in objects {
                        reality.spawn_object(object);
                    }
                }
                Packet::ChatMessage(message) => {
                    reality.queue_chat(message);
                }
                p => {
                    panic!("Unhandled client packet failed netty! ({:?})", p);
                }
            }
        }
    }
    pub fn system_startup_checks(
        mut netty: ResMut<Netty>,
        mut state: ResMut<State<GameState>>,
        disk: Res<Disk>
    ) {
        match netty.connection() {
            ConnectionStatus::Connected | ConnectionStatus::Stable => {
                if disk.user().is_some() {
                    info!("Logging in user");
                    netty.say(Packet::UserPresence(disk.user().unwrap()));
                    state.set(GameState::TitleScreen).unwrap();
                }
                else {
                    info!("Opening user creation screen");
                    state.set(GameState::MakeUser).unwrap();
                }
            }
            _ => {
                info!("No network connection");
                state.set(GameState::OfflineTitle).unwrap();
            }
        }
    }
    pub fn system_step(
        mut netty: ResMut<Netty>,
        mut reality: ResMut<Reality>,
        mut disk: ResMut<Disk>,
    ) {
        netty.step(&mut reality, &mut disk);
    }
    pub fn system_server_list(
        mut netty: ResMut<Netty>,
    ) {
        netty.say(Packet::AvalableServers)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectionStatus {
    NoInternet,
    NoGGS,
    Connected,
    Stable
}

pub fn google_exists() -> bool {
    std::net::TcpStream::connect_timeout(
        &"172.217.1.110:80".parse().unwrap(),
        std::time::Duration::from_secs(5)
    ).is_ok()
}

fn startup(mut con: TcpStream, recv_buffer: Arc<Mutex<Vec<Packet>>>, send_buffer: Arc<Mutex<Vec<Packet>>>) {
    info!("Starting client with GGS set to {:?}.", con.peer_addr());
    info!("GGS located at {:?}:{}", GGS, NETTY_PORT);
    info!("NETTY VERSION: {}", NETTY_VERSION);
    let mut con_clone = con.try_clone().unwrap();
    std::thread::spawn(move || {
        loop {
            let mut send_access = send_buffer.lock().unwrap();
            for packet in send_access.iter() {
                Packet::to_write(&mut con_clone, packet.clone());
            }
            send_access.clear();
            drop(send_access);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    });
    std::thread::spawn(move || {
        loop {
            let pkt = Packet::from_read(&mut con);
            let mut recv_access = recv_buffer.lock().unwrap();
            if pkt == Packet::FailedDeserialize {
                todo!("Remote dead? Proper handle needed.");
            }
            recv_access.push(pkt);
            drop(recv_access);
        }
    });
}