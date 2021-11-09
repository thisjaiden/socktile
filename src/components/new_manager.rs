use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::shared::{netty::Packet, saves::{save_user, user}, world::World};

#[derive(Clone, Debug)]
pub struct NewManager {
    entity_ids: Vec<Entity>,
    net_in: Arc<Mutex<Vec<Packet>>>,
    net_out: Arc<Mutex<Vec<Packet>>>,
    waiting_for_net: bool,
    time_to_swap: bool,
    world: Option<World>
}

impl NewManager {
    pub fn new(entity_ids: Vec<Entity>, net_in: Arc<Mutex<Vec<Packet>>>, net_out: Arc<Mutex<Vec<Packet>>>) -> Self {
        Self {
            entity_ids,
            net_in,
            net_out,
            waiting_for_net: false,
            time_to_swap: false,
            world: None
        }
    }
    pub fn grab_in(&mut self) -> Arc<Mutex<Vec<Packet>>> {
        self.net_in.clone()
    }
    pub fn grab_out(&mut self) -> Arc<Mutex<Vec<Packet>>> {
        self.net_out.clone()
    }
    pub fn grab_world(&mut self) -> World {
        self.world.clone().unwrap()
    }
    pub fn add_eid(&mut self, eid: Entity) {
        self.entity_ids.push(eid);
    }
    pub fn net_mode(&mut self) {
        self.waiting_for_net = true;
    }
    pub fn is_waiting(&mut self) -> bool {
        self.waiting_for_net
    }
    pub fn swap_time(&mut self) -> bool {
        self.time_to_swap
    }
    pub fn network_step(&mut self) {
        let input = self.grab_in();
        let mut input_access = input.lock().unwrap();
        let recieved_packets = input_access.clone();
        input_access.clear();
        drop(input_access);
        drop(input);
        for packet in recieved_packets {
            match packet {
                Packet::CreatedProfile(profile) => {
                    save_user(profile.user);
                }
                Packet::CreatedWorld(iid) => {
                    let output = self.grab_out();
                    let mut output_access = output.lock().unwrap();
                    output_access.push(Packet::JoinWorld(iid, user().unwrap()));
                    drop(output_access);
                    drop(output);
                }
                Packet::FullWorldData(world) => {
                    self.world = Some(world);
                    self.time_to_swap = true;
                }
                p => {
                    println!("Unexpected packet {:?} during New!", p);
                }
            }
        }
    }
    pub fn disassemble(&mut self, commands: &mut Commands) {
        for entity in &self.entity_ids {
            commands.entity(*entity).despawn();
        }
    }
}
