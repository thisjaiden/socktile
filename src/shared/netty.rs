use std::{net::SocketAddr, sync::{Arc, Mutex}};

use crate::{client::core::GGS, components::GamePosition};

use super::{saves::{Profile, User}, world::World};
use serde::{Deserialize, Serialize};

pub const NETTY_VERSION: &str = "closed-alpha-iteration-0";

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub enum Packet {
    /// Post the version of the network protocol being used.
    /// A response is expected, regardless of the version on either end.
    /// TODO: This hasn't been confirmed to work cross-version, `bincode` enums are a black box.
    /// (Netty Version)
    NettyVersion(String),
    /// The server uses the same version. Continue.
    /// (No Data)
    SameVersion,
    /// The server uses a different version. Do not connect.
    /// (No Data)
    DifferentVerison,
    /// Data was recieved but unable to be deseriZalized.
    /// (No Data)
    FailedDeserialize,
    /// Request a profile from the remote server.
    /// (User)
    RequestProfile(User),
    /// Server responds with a profile for the user.
    /// (Profile)
    GiveProfile(Profile),
    /// Server responds with no profile. (Does not exist/Not found)
    /// (No Data)
    NoProfile,
    /// Create a profile on the remote server.
    /// (User)
    CreateProfile(User),
    /// Confirm creation of a profile.
    /// NOTE: The user's tag of the User in this profile can be different from that of the requested
    /// profile. Take this into account.
    /// (Profile)
    CreatedProfile(Profile),
    /// Create a world on the remote server.
    /// (World Name)
    CreateWorld(String),
    /// Confirm creation of a world.
    /// (World ID)
    CreatedWorld(usize),
    /// Request to join a world.
    /// (World ID, User)
    JoinWorld(usize, User),
    /// Disconnects from a world.
    /// (No Data)
    LeaveWorld,
    /// Mainly used when joining a world. A complete structure of all data. This is a lot, don't
    /// just send this whenever.
    /// (World)
    FullWorldData(World),
    /// Requests moving a player to a new position in a world.
    /// (New Position)
    RequestMove(GamePosition),
    /// Updates the positions of any players who have moved.
    /// (Array (Player, New Position))
    PlayerPositionUpdates(Vec<(User, GamePosition)>)
}

impl Packet {
    pub fn from_read<R: std::io::Read>(read: &mut R) -> Packet {
        bincode::deserialize_from(read).unwrap_or(Packet::FailedDeserialize)
    }
    pub fn to_write<W: std::io::Write>(write: &mut W, packet: Packet) {
        write.write_all(&bincode::serialize(&packet).unwrap()).unwrap();
    }
}

pub fn remote_exists() -> bool {
    if online::sync::check(Some(5)).is_ok() {
        if std::net::TcpStream::connect_timeout(&GGS.parse().unwrap(), std::time::Duration::from_secs(5)).is_ok() {
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

pub fn initiate_slave(remote: &str, recv_buffer: Arc<Mutex<Vec<Packet>>>, send_buffer: Arc<Mutex<Vec<Packet>>>) -> ! {
    if !remote_exists() {
        todo!("No network, offline mode");
    }
    let mut con = std::net::TcpStream::connect(remote).unwrap();
    let mut con_clone = con.try_clone().unwrap();
    Packet::to_write(&mut con, Packet::NettyVersion(String::from(NETTY_VERSION)));
    std::thread::spawn(move || {
        loop {
            let mut send_access = send_buffer.lock().unwrap();
            for packet in send_access.iter() {
                println!("Sending {:?} to GGS", packet);
                Packet::to_write(&mut con_clone, packet.clone());
            }
            send_access.clear();
            drop(send_access);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    });
    loop {
        let pkt = Packet::from_read(&mut con);
        let mut recv_access = recv_buffer.lock().unwrap();
        println!("Recieved {:?} from GGS", pkt);
        if pkt == Packet::DifferentVerison {
            todo!("Invalid version, offline mode");
        }
        if pkt == Packet::FailedDeserialize {
            todo!("Remote dead? Proper handle needed.");
        }
        recv_access.push(pkt);
        drop(recv_access);
    }
}


pub fn initiate_host(recv_buffer: Arc<Mutex<Vec<(Packet, SocketAddr)>>>, send_buffer: Arc<Mutex<Vec<(Packet, SocketAddr)>>>) -> ! {
    let net = std::net::TcpListener::bind(format!("0.0.0.0:{}", crate::server::core::HOST_PORT));
    if let Ok(network) = net {
        for connection in network.incoming() {
            if let Ok(mut stream) = connection {
                let recv_clone = recv_buffer.clone();
                let send_clone = send_buffer.clone();
                let remote_addr = stream.peer_addr().unwrap();
                let mut stream_clone = stream.try_clone().unwrap();
                std::thread::spawn(move || {
                    let recv = recv_clone;
                    loop {
                        let pkt = Packet::from_read(&mut stream);
                        let mut recv_access = recv.lock().unwrap();
                        println!("Recieved {:?} from {:?}", pkt, remote_addr);
                        if pkt == Packet::FailedDeserialize {
                            println!("Dropping connection to {:?}", remote_addr);
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
                        for (packet, address) in send_access.iter() {
                            println!("Sending {:?} to {}", packet, address);
                            if packet == &Packet::DifferentVerison || packet == &Packet::FailedDeserialize {
                                destroy_conenction = true;
                            }
                            if address == &remote_addr {
                                Packet::to_write(&mut stream_clone, packet.clone());
                            }
                        }
                        send_access.clear();
                        drop(send_access);
                        if destroy_conenction {
                            println!("Dropping connection to {:?}", remote_addr);
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
        panic!("Network reads blocked!");
    }
    unreachable!();
}
