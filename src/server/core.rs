use std::sync::{Arc, Mutex};

use crate::shared::{netty::initiate_host, saves::{saves, users}};

pub const HOST_PORT: &str = "6969";

pub fn startup() -> ! {
    println!("STARTING GLOBAL GAME SERVER WITH PORT {}. PLEASE CLOSE ALL OTHER INSTANCES OF THE GGS AND APPLICATIONS USING THIS PORT.", HOST_PORT);
    let recv = Arc::new(Mutex::new(vec![]));
    let send = Arc::new(Mutex::new(vec![]));
    let recv_clone = recv.clone();
    let send_clone = send.clone();
    std::thread::spawn(move || {
        initiate_host(recv_clone, send_clone);
    });
    let mut timer = std::time::Instant::now();
    let mut saves = saves();
    let mut profiles = users();
    loop {
        if timer.elapsed() > std::time::Duration::from_millis(50) {
            let mut func_recv = recv.lock().unwrap();
            let incoming_data = func_recv.clone();
            func_recv.clear();
            drop(func_recv);
            for (packet, from) in incoming_data {
                println!("{}: {:?}", from, packet);
            }
            timer = std::time::Instant::now();
        }
        else {
            std::thread::sleep(std::time::Duration::from_millis(50) - timer.elapsed());
        }
    }
}
