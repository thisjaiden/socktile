use std::{net::SocketAddr, sync::{Arc, Mutex}};

use crate::components::GamePosition;

use super::{saves::{Profile, SaveGame, User}, world::World};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub enum Packet {
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
    /// (Internal World Name)
    CreatedWorld(String),
    /// Request to join a world.
    /// (Internal World Name, User)
    JoinWorld(String, User),
    /// Mainly used when joining a world. A complete structure of all data. This is a lot, don't
    /// just send this whenever.
    /// (World)
    FullWorldData(World),
    /// Requests moving a player to a new position in a world.
    /// (Talk UUID, Position)
    RequestMove(u128, GamePosition)
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
        true
    }
    else {
        println!("No internet connection avalable.");
        false
    }
}

pub fn initiate_slave(remote: &str, recv_buffer: Arc<Mutex<Vec<Packet>>>, send_buffer: Arc<Mutex<Vec<Packet>>>) -> ! {
    let mut con = std::net::TcpStream::connect(remote).unwrap();
    loop {
        let mut send_access = send_buffer.lock().unwrap();
        for packet in send_access.iter() {
            println!("Writing {:?} to network.", packet);
            Packet::to_write(&mut con, packet.clone());
        }
        send_access.clear();
        drop(send_access);
        let pkt = Packet::from_read(&mut con);
        let mut recv_access = recv_buffer.lock().unwrap();
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
                std::thread::spawn(move || {
                    let recv = recv_clone;
                    let send = send_clone;
                    let remote_addr = stream.peer_addr().unwrap();
                    loop {
                        let pkt = Packet::from_read(&mut stream);
                        let mut recv_access = recv.lock().unwrap();
                        recv_access.push((pkt, remote_addr));
                        drop(recv_access);
                        let send_access = send.lock().unwrap();
                        for (packet, address) in send_access.iter() {
                            if address == &remote_addr {
                                Packet::to_write(&mut stream, packet.clone());
                            }
                        }
                        drop(send_access);
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
    loop {}
}
