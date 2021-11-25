use crate::{client::core::{GGS, startup}, components::NewManager, shared::{netty::{NETTY_VERSION, Packet, remote_exists}, saves::save_user}};

use std::{net::TcpStream, sync::{Arc, Mutex}};

pub struct Netty {
    connection: ConnectionStatus,
    internet_access: bool,
    ggs_access: bool,
    input: Arc<Mutex<Vec<Packet>>>,
    output: Arc<Mutex<Vec<Packet>>>,
    pool_queues: Vec<(String, Packet)>
}

impl Netty {
    pub fn init() -> Netty {
        println!("Netty initalizing!");
        let internet = online::sync::check(Some(5)).is_ok();
        let ggs = remote_exists();
        let connection = TcpStream::connect(GGS);
        let mut stat = ConnectionStatus::NotConnected;
        if !ggs {
            stat = ConnectionStatus::NoGGS;
        }
        if !internet {
            stat = ConnectionStatus::NoInternet;
        }
        if stat != ConnectionStatus::NotConnected {
            return Netty {
                internet_access: internet,
                ggs_access: ggs,
                connection: stat,
                input: Arc::new(Mutex::new(vec![])),
                output: Arc::new(Mutex::new(vec![])),
                pool_queues: vec![]
            };
        }
        if let Ok(good_con) = connection {
            let inp = Arc::new(Mutex::new(vec![]));
            let out = Arc::new(Mutex::new(vec![]));
            println!("Good connection to GGS, starting up client.");
            startup(good_con, inp.clone(), out.clone());
            let mut fin = Netty {
                internet_access: internet,
                ggs_access: ggs,
                connection: ConnectionStatus::Connected,
                input: inp,
                output: out,
                pool_queues: vec![]
            };
            fin.say(Packet::NettyVersion(String::from(NETTY_VERSION)));
            fin
        }
        else {
            println!("GGS refused a connection. Not starting client.");
            Netty {
                internet_access: internet,
                ggs_access: false,
                connection: ConnectionStatus::Refused,
                input: Arc::new(Mutex::new(vec![])),
                output: Arc::new(Mutex::new(vec![])),
                pool_queues: vec![]
            }
        }
    }
    pub fn ggs_connection(&mut self) -> bool {
        self.ggs_access
    }
    pub fn internet_connection(&mut self) -> bool {
        self.internet_access
    }
    pub fn say(&mut self, packet: Packet) {
        let mut out = self.output.lock().unwrap();
        out.push(packet);
        drop(out);
    }
    pub fn exclusive_tick(&mut self) {
        let mut input = self.input.lock().unwrap();
        let pkts = input.clone();
        input.clear();
        drop(input);
        for packet in pkts {
            match packet {
                Packet::CreatedProfile(prof) => {
                    save_user(prof.user);
                }
                Packet::DifferentVerison => {
                    self.connection = ConnectionStatus::Old;
                    self.ggs_access = false;
                }
                Packet::CreatedWorld(_) => {
                    self.pool_queues.push((String::from("new"), packet));
                }
                p => {
                    panic!("Unhandled client packet failed netty! ({:?})", p);
                }
            }
        }
    }
    pub fn new_tick(&mut self, man: &mut NewManager) {
        for (pool, packet) in self.pool_queues.clone() {
            if pool == "new" {
                man.net_mode();
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectionStatus {
    NoInternet,
    NoGGS,
    Refused,
    NotConnected,
    Old,
    Connected
}
