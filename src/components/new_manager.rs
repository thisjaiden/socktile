use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::shared::{netty::Packet, saves::{Profile, save_user}};

#[derive(Clone, Debug)]
pub struct NewManager {
    entity_ids: Vec<Entity>,
    net_in: Arc<Mutex<Vec<Packet>>>,
    net_out: Arc<Mutex<Vec<Packet>>>,
    profile: Option<Profile>,
    has_ui: bool
}

impl NewManager {
    pub fn new(entity_ids: Vec<Entity>, net_in: Arc<Mutex<Vec<Packet>>>, net_out: Arc<Mutex<Vec<Packet>>>) -> Self {
        Self {
            entity_ids,
            net_in,
            net_out,
            profile: None,
            has_ui: false
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
    pub fn has_profile(&mut self) -> bool {
        self.profile.is_some()
    }
    pub fn has_ui(&mut self) -> bool {
        self.has_ui
    }
    pub fn set_ui_state(&mut self, state: bool) {
        self.has_ui = state;
    }
    pub fn add_eid(&mut self, eid: Entity) {
        self.entity_ids.push(eid);
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
                Packet::CreatedProfile(profile) => {
                    self.profile = Some(profile.clone());
                    save_user(profile.user);
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
