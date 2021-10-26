use std::{net::SocketAddr, sync::{Arc, Mutex}};

use super::{saves::{Profile, User}};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub enum Packet {
    /// Data was recieved but unable to be deserialized.
    /// (No Data)
    FailedDeserialize,
    /// Request a profile from the remote server.
    /// (User)
    RequestProfile(User),
    /// Server responds with a profile for the user.
    /// (Profile)
    GiveProfile(Profile),
    /// Create a profile on the remote server.
    /// (User)
    CreateProfile(User),
    /// Confirm creation of a profile.
    /// NOTE: The user's tag of the User in this profile can be different from that of the requested
    /// profile. Take this into account.
    /// (Profile)
    CreatedProfile(Profile),
}

impl Packet {
    pub fn from_read<R: std::io::Read>(read: &mut R) -> Packet {
        bincode::deserialize_from(read).unwrap_or(Packet::FailedDeserialize)
    }
    pub fn to_write<W: std::io::Write>(write: &mut W, packet: Packet) {
        write.write_all(&bincode::serialize(&packet).unwrap()).unwrap();
    }
}

pub fn initiate_slave(remote: &str, recv_buffer: Arc<Mutex<Vec<Packet>>>, send_buffer: Arc<Mutex<Vec<Packet>>>) -> ! {
    let mut con = std::net::TcpStream::connect(remote).unwrap();
    loop {
        let pkt = Packet::from_read(&mut con);
        let mut recv_access = recv_buffer.lock().unwrap();
        recv_access.push(pkt);
        drop(recv_access);
        let mut send_access = send_buffer.lock().unwrap();
        for packet in send_access.iter() {
            Packet::to_write(&mut con, packet.clone());
        }
        send_access.clear();
        drop(send_access);
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
