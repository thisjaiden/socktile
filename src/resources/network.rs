use netty::client::{Client, ClientConfig};

use super::{chat::ChatMessage, Reality};
use crate::prelude::*;

#[cfg(target_arch = "wasm32")]
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct Netty {
    #[cfg(not(target_arch = "wasm32"))]
    n: Client<Packet>,
    #[cfg(target_arch = "wasm32")]
    n: Arc<Mutex<Client<Packet>>>,
    #[cfg(target_arch = "wasm32")]
    buffer: Vec<Packet>,
}

impl Netty {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(n: Client<Packet>) -> Netty {
        Netty {
            n
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn new(n: Arc<Mutex<Client<Packet>>>) -> Netty {
        Netty {
            n,
            buffer: vec![]
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn send(&mut self, p: Packet) {
        let mut ax = self.n.lock().unwrap();
        // TODO: this is a particularally wasteful clone. Don't care!
        let success = ax.send(p.clone()).is_ok();
        drop(ax);
        if !success {
            self.buffer.push(p);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn send(&mut self, p: Packet) {
        self.n.send(p);
    }
    pub fn update(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            let cl = self.buffer.clone();
            self.buffer.clear();
            for pkt in cl {
                self.send(pkt);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn init() -> Option<Netty> {
    info!("Netty initalizing");

    let client_attempt = Client::launch(ClientConfig {
        address: GGS,
        tcp_port: TCP_PORT,
        ws_port: WS_PORT,
        connection_timeout: TIMEOUT_DURATION,
        ..default()
    });
    if let Some(client) = client_attempt {
        info!("Good connection to GGS, Netty constructed");
        let mut n = Netty::new(client);
        n.send(Packet::NettyVersion(String::from(NETTY_VERSION)));
        Some(n)
    }
    else {
        warn!("Unable to construct Netty.");
        None
    }
}

#[cfg(target_arch = "wasm32")]
fn init() -> Option<Netty> {
    info!("Netty initalizing");

    let client = Client::launch(ClientConfig {
        address: GGS,
        tcp_port: TCP_PORT,
        ws_port: WS_PORT,
        connection_timeout: TIMEOUT_DURATION,
        ..default()
    });
    let mut n = Netty::new(client);
    n.send(Packet::NettyVersion(String::from(NETTY_VERSION)));
    Some(n)
}

pub fn system_startup_checks(
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    disk: Res<Disk>,
) {
    let pot_client = init();
    if let Some(mut client) = pot_client {
        if disk.user().is_some() {
            info!("Logging in user");
            client.send(Packet::UserPresence(disk.user().unwrap()));
            state.set(GameState::TitleScreen);
        }
        else {
            info!("Opening user creation screen");
            state.set(GameState::MakeUser);
        }
        commands.insert_resource(client);
    }
    else {
        warn!("No network connection");
        state.set(GameState::OfflineTitle);
    }
}

pub fn system_step(
    netty: Option<ResMut<Netty>>,
    mut reality: ResMut<Reality>,
    mut disk: ResMut<Disk>,
) {
    if let Some(mut netty) = netty {
        netty.update();
        let pkts;
        #[cfg(not(target_arch = "wasm32"))]
        {
            pkts = netty.n.get_packets();
        }
        #[cfg(target_arch = "wasm32")]
        {
            let mut ax = netty.n.lock().unwrap();
            pkts = ax.get_packets();
            drop(ax);
        }
        for packet in pkts {
            match packet {
                Packet::CreatedUser(user) => {
                    while !disk.update_user(user.clone()) {}
                    info!("Saved new user information.");
                }
                Packet::AllSet => {
                    // do nothing
                }
                Packet::CreatedWorld(id) => {
                    netty.send(Packet::JoinWorld(id));
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
                        sent_at: std::time::Instant::now(),
                    });
                }
                Packet::NoWhitelistPermission => {
                    reality.queue_chat(ChatMessage {
                        text: String::from("You don't have permission to whitelist other users."),
                        color: Color::RED,
                        sent_at: std::time::Instant::now(),
                    });
                }
                Packet::UnwhitelistableUser => {
                    reality.queue_chat(ChatMessage {
                        text: String::from(
                            "Unable to whitelist user. (Did you spell everything right?)",
                        ),
                        color: Color::RED,
                        sent_at: std::time::Instant::now(),
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
                Packet::TileUpdate(chunk, tile, state) => {
                    reality.update_tile(chunk, tile, state)
                }
                p => {
                    panic!("Unhandled client packet failed netty! ({:?})", p);
                }
            }
        }
    }
}

pub fn system_server_list(mut netty: ResMut<Netty>) {
    netty.send(Packet::AvalableServers)
}
