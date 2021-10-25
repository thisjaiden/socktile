use std::sync::{Arc, Mutex};

use crate::shared::{netty::{Packet, initiate_slave}};

const GGS: &str = "lumen.place:6666";

pub fn startup(recv: Arc<Mutex<Vec<Packet>>>, send: Arc<Mutex<Vec<Packet>>>) -> ! {
    println!("Starting client with GGS set to {}.", GGS);
    initiate_slave(GGS, recv, send);
}
