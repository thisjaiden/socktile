use std::{net::TcpStream, sync::{Arc, Mutex}};

use crate::shared::netty::{NETTY_VERSION, Packet};

pub const GGS: &str = "lumen.place:11111";
pub const DEV_GGS: &str = "127.0.0.1:11111";
const DEBUG_SPAM: bool = false;

pub fn startup(connection: TcpStream, recv: Arc<Mutex<Vec<Packet>>>, send: Arc<Mutex<Vec<Packet>>>) {
    println!("Starting client with GGS set to {:?}.", connection.peer_addr());
    println!("GGS | DEV_GGS: {} | {}", GGS, DEV_GGS);
    initiate_slave(connection, recv, send);
}

fn initiate_slave(mut con: TcpStream, recv_buffer: Arc<Mutex<Vec<Packet>>>, send_buffer: Arc<Mutex<Vec<Packet>>>) {
    println!("NETTY VERSION: {}", NETTY_VERSION);
    let mut con_clone = con.try_clone().unwrap();
    std::thread::spawn(move || {
        loop {
            let mut send_access = send_buffer.lock().unwrap();
            for packet in send_access.iter() {
                if DEBUG_SPAM {
                    println!("Sending {:?} to GGS", packet);
                }
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
            println!("Recieved {:?} from GGS", pkt);
            if pkt == Packet::FailedDeserialize {
                todo!("Remote dead? Proper handle needed.");
            }
            recv_access.push(pkt);
            drop(recv_access);
        }
    });
}
