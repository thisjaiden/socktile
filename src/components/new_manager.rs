use bevy::prelude::*;

use crate::shared::world::World;

#[derive(Clone, Debug)]
pub struct NewManager {
    entity_ids: Vec<Entity>,
    waiting_for_net: bool,
    time_to_swap: bool,
    world: Option<World>
}

impl NewManager {
    pub fn new(entity_ids: Vec<Entity>) -> Self {
        Self {
            entity_ids,
            waiting_for_net: false,
            time_to_swap: false,
            world: None
        }
    }
    pub fn grab_world(&mut self) -> World {
        self.world.clone().unwrap()
    }
    pub fn net_mode(&mut self) {
        if !self.waiting_for_net {
            self.waiting_for_net = true;
        }
        else {
            self.time_to_swap = true;
        }
    }
    pub fn is_waiting(&mut self) -> bool {
        self.waiting_for_net
    }
    pub fn swap_time(&mut self) -> bool {
        self.time_to_swap
    }
    pub fn disassemble(&mut self, commands: &mut Commands) {
        for entity in &self.entity_ids {
            commands.entity(*entity).despawn();
        }
    }
}
