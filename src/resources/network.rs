use netty::client::{Client, ClientConfig};

use crate::prelude::*;
use super::{Reality, chat::ChatMessage};

pub fn init() -> Option<Client<Packet>> {
    info!("Netty initalizing");

    let client_attempt = Client::launch(ClientConfig {
        address: GGS,
        tcp_port: TCP_PORT,
        ws_port: WS_PORT,
        connection_timeout: TIMEOUT_DURATION,
        ..default()
    });
    if let Some(mut client) = client_attempt {
        info!("Good connection to GGS, Netty constructed");
        client.send(Packet::NettyVersion(String::from(NETTY_VERSION)));
        Some(client)
    }
    else {
        None
    }
}

pub fn system_startup_checks(
    o_netty: Option<ResMut<Client<Packet>>>,
    mut state: ResMut<State<GameState>>,
    disk: Res<Disk>
) {
    if let Some(mut netty) = o_netty {
        if disk.user().is_some() {
            info!("Logging in user");
            netty.send(Packet::UserPresence(disk.user().unwrap()));
            state.overwrite_set(GameState::TitleScreen).unwrap();
        }
        else {
            info!("Opening user creation screen");
            state.overwrite_set(GameState::MakeUser).unwrap();
        }
    }
    else {
        info!("No network connection");
        state.overwrite_set(GameState::OfflineTitle).unwrap();
    }
}

pub fn system_step(
    mut netty: ResMut<Client<Packet>>,
    mut reality: ResMut<Reality>,
    mut disk: ResMut<Disk>,
) {
    let pkts = netty.get_packets();
    for packet in pkts {
        match packet {
            Packet::CreatedUser(user) => {
                while !disk.update_user(user.clone()) {};
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

pub fn system_server_list(
    mut netty: ResMut<Client<Packet>>,
) {
    netty.send(Packet::AvalableServers)
}
