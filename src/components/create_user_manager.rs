use bevy::prelude::*;

use crate::{resources::GameState};

#[derive(Clone, Debug)]
pub struct CreateUserManager {
    entity_ids: Vec<Entity>,
    previous_state: GameState
}

impl CreateUserManager {
    pub fn new(entity_ids: Vec<Entity>) -> Self {
        Self {
            entity_ids,
            previous_state: GameState::Join
        }
    }
    pub fn new_b(entity_ids: Vec<Entity>) -> Self {
        Self {
            entity_ids,
            previous_state: GameState::New
        }
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
