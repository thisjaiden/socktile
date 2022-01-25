use crate::{GameState, consts::*, shared::{netty::Packet, saves::save_user}};

use std::{net::TcpStream, sync::{Arc, Mutex}};

use bevy::prelude::*;

use super::Reality;

pub struct Netty {
    connection: ConnectionStatus,
    input: Arc<Mutex<Vec<Packet>>>,
    output: Arc<Mutex<Vec<Packet>>>,
}

impl Netty {
    pub fn init() -> Netty {
        println!("Netty initalizing!");

        let l_ggs = if DEV_BUILD {
            DEV_GGS
        }
        else {
            GGS
        };

        let connection = TcpStream::connect(l_ggs);
        let mut stat = ConnectionStatus::NotConnected;
        if !remote_exists(l_ggs) {
            stat = ConnectionStatus::NoGGS;
        }
        if online::sync::check(Some(5)).is_err() {
            stat = ConnectionStatus::NoInternet;
        }
        if stat == ConnectionStatus::NoInternet && DEV_BUILD {
            if let Ok(good_con) = connection {
                let inp = Arc::new(Mutex::new(vec![]));
                let out = Arc::new(Mutex::new(vec![]));
                println!("Good connection to GGS (LOCAL-DEV), starting up client.");
                startup(good_con, inp.clone(), out.clone());
                let mut fin = Netty {
                    connection: ConnectionStatus::Connected,
                    input: inp,
                    output: out
                };
                fin.say(Packet::NettyVersion(String::from(NETTY_VERSION)));
                return fin;
            }
            else {
                println!("GGS refused a connection. Not starting client. (NO_INTERNET)");
                return Netty {
                    connection: ConnectionStatus::Refused,
                    input: Arc::new(Mutex::new(vec![])),
                    output: Arc::new(Mutex::new(vec![]))
                };
            }
        }
        else if stat != ConnectionStatus::NotConnected {
            println!("Unable to connect to GGS, not starting client. (ERR: {:?})", stat);
            return Netty {
                connection: stat,
                input: Arc::new(Mutex::new(vec![])),
                output: Arc::new(Mutex::new(vec![]))
            };
        }
        if let Ok(good_con) = connection {
            let inp = Arc::new(Mutex::new(vec![]));
            let out = Arc::new(Mutex::new(vec![]));
            println!("Good connection to GGS, starting up client.");
            startup(good_con, inp.clone(), out.clone());
            let mut fin = Netty {
                connection: ConnectionStatus::Connected,
                input: inp,
                output: out
            };
            fin.say(Packet::NettyVersion(String::from(NETTY_VERSION)));
            fin
        }
        else {
            println!("GGS refused a connection. Not starting client.");
            Netty {
                connection: ConnectionStatus::Refused,
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
    pub fn step(&mut self, reality: &mut Reality) {
        let mut input = self.input.lock().unwrap();
        let pkts = input.clone();
        input.clear();
        drop(input);
        for packet in pkts {
            match packet {
                Packet::CreatedUser(user) => {
                    save_user(user);
                    println!("Saved new user information.");
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
                Packet::ChangesChunk(chunk, changes) => {
                    reality.add_chunk(chunk, changes);
                    reality.update_chunk(chunk);
                }
                Packet::ServerList(servers) => {
                    reality.set_avalable_servers(servers);
                }
                Packet::WrongVersion(prefered_version) => {
                    panic!("Server is running {}, and you're using {} (You're most likely out of date, update!)", prefered_version, NETTY_VERSION);
                }
                Packet::OnlinePlayers(players) => {
                    reality.add_online_players(players);
                }
                Packet::PlayerPositionUpdate(p, l) => {
                    reality.queue_player_move(p, l)
                }
                p => {
                    panic!("Unhandled client packet failed netty! ({:?})", p);
                }
            }
        }
    }
    pub fn system_startup_checks(
        mut netty: ResMut<Netty>,
        mut state: ResMut<State<GameState>>
    ) {
        match netty.connection() {
            ConnectionStatus::Connected | ConnectionStatus::Stable => {
                state.set(GameState::TitleScreen).unwrap();
            }
            _ => {
                state.set(GameState::OfflineTitle).unwrap();
            }
        }
    }
    pub fn system_step(
        mut netty: ResMut<Netty>,
        mut reality: ResMut<Reality>
    ) {
        netty.step(&mut reality);
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
    Refused,
    NotConnected,
    Connected,
    Stable
}

pub fn remote_exists(ggs: &str) -> bool {
    if online::sync::check(Some(5)).is_ok() {
        if std::net::TcpStream::connect_timeout(&ggs.parse().unwrap(), std::time::Duration::from_secs(5)).is_ok() {
            true
        }
        else {
            println!("No connection to the GGS avalable.");
            false
        }
    }
    else {
        println!("No internet connection avalable.");
        false
    }
}

fn startup(mut con: TcpStream, recv_buffer: Arc<Mutex<Vec<Packet>>>, send_buffer: Arc<Mutex<Vec<Packet>>>) {
    println!("Starting client with GGS set to {:?}.", con.peer_addr());
    println!("GGS | DEV_GGS: {} | {}", GGS, DEV_GGS);
    println!("NETTY VERSION: {}", NETTY_VERSION);
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