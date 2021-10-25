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
    pub fn grab_in(self) -> Arc<Mutex<Vec<Packet>>> {
        return self.net_in.clone();
    }
    pub fn grab_out(self) -> Arc<Mutex<Vec<Packet>>> {
        return self.net_out.clone();
    }
    pub fn grab_profile(self) -> Option<Profile> {
        return self.profile.clone();
    }
    pub fn network_step(&mut self) {

    }
    pub fn disassemble(&mut self, commands: &mut Commands) {
        for entity in &self.entity_ids {
            commands.entity(*entity).despawn();
        }
    }
}
