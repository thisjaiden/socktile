use std::sync::{Arc, Mutex};

use crate::shared::{netty::{Packet, initiate_slave}};

pub const GGS: &str = "127.0.0.1:11111";

pub fn startup(recv: Arc<Mutex<Vec<Packet>>>, send: Arc<Mutex<Vec<Packet>>>) -> ! {
    println!("Starting client with GGS set to {}.", GGS);
    initiate_slave(GGS, recv, send);
}
