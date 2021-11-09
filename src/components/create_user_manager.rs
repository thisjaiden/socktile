use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::{resources::GameState, shared::netty::Packet};

#[derive(Clone, Debug)]
pub struct CreateUserManager {
    net_in: Arc<Mutex<Vec<Packet>>>,
    net_out: Arc<Mutex<Vec<Packet>>>,
    entity_ids: Vec<Entity>,
    previous_state: GameState
}

impl CreateUserManager {
    pub fn new(entity_ids: Vec<Entity>, net_in: Arc<Mutex<Vec<Packet>>>, net_out: Arc<Mutex<Vec<Packet>>>) -> Self {
        Self {
            entity_ids,
            net_in,
            net_out,
            previous_state: GameState::Join
        }
    }
    pub fn new_b(entity_ids: Vec<Entity>, net_in: Arc<Mutex<Vec<Packet>>>, net_out: Arc<Mutex<Vec<Packet>>>) -> Self {
        Self {
            entity_ids,
            net_in,
            net_out,
            previous_state: GameState::New
        }
    }
    pub fn grab_in(&mut self) -> Arc<Mutex<Vec<Packet>>> {
        self.net_in.clone()
    }
    pub fn grab_out(&mut self) -> Arc<Mutex<Vec<Packet>>> {
        self.net_out.clone()
    }
    pub fn grab_previous_state(&mut self) -> GameState {
        self.previous_state
    }
    pub fn disassemble(&mut self, commands: &mut Commands) {
        for entity in &self.entity_ids {
            commands.entity(*entity).despawn();
        }
    }
}
