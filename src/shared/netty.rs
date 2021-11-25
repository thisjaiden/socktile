use std::{net::SocketAddr, sync::{Arc, Mutex}};

use crate::components::GamePosition;

use super::{object::Object, saves::User, terrain::TerrainState, world::World};
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
    /// Data was recieved but unable to be deserizalized.
    /// (No Data)
    FailedDeserialize,
    /// Create a user on the remote server.
    /// (User)
    CreateUser(User),
    /// Confirm creation of a profile.
    /// NOTE: The user's tag of the User in this profile should be different from that of the requested
    /// profile. Take this into account.
    /// (User)
    CreatedUser(User),
    /// Lets the server know what user is associated with what IP.
    /// (User)
    UserPresence(User),
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
    /// A client has been connected. Send them their position.
    /// (Player Position)
    JoinedGame(GamePosition),
    /// The server sends over the information relating to some terrain.
    /// This is always a 64x64 chunk.
    /// (Chunk Position, Chunk Data)
    TerrainChunk((usize, usize), Vec<TerrainState>),
    /// Sends over all game objects.
    /// (Game Objects)
    AllObjects(Vec<Object>),
    /// Updates a given object on the client.
    /// (Updated Object)
    UpdateObject(Object),
    /// Removes an object on the client by id.
    /// (Object ID)
    RemoveObject(usize),
    /// Creates an object on the client.
    /// (New Object)
    CreateObject(Object),
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
