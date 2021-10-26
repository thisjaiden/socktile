use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::shared::{netty::Packet, saves::Profile};

#[derive(Clone, Debug)]
pub struct JoinManager {
    entity_ids: Vec<Entity>,
    net_in: Arc<Mutex<Vec<Packet>>>,
    net_out: Arc<Mutex<Vec<Packet>>>,
    profile: Option<Profile>
}

impl JoinManager {
    pub fn new(entity_ids: Vec<Entity>, net_in: Arc<Mutex<Vec<Packet>>>, net_out: Arc<Mutex<Vec<Packet>>>) -> Self {
        Self {
            entity_ids,
            net_in,
            net_out,
            profile: None
        }
    }
    pub fn grab_in(&mut self) -> Arc<Mutex<Vec<Packet>>> {
        self.net_in.clone()
    }
    pub fn grab_out(&mut self) -> Arc<Mutex<Vec<Packet>>> {
        self.net_out.clone()
    }
    pub fn grab_profile(&mut self) -> Option<Profile> {
        self.profile.clone()
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
                Packet::GiveProfile(profile) => {
                    self.profile = Some(profile);
                }
                p => {
                    println!("Unexpected packet {:?} during Join!", p);
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
