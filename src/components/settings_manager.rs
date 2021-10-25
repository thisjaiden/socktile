use bevy::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingsManager {
    entity_ids: Vec<Entity>
}

impl SettingsManager {
    pub fn new(entity_ids: Vec<Entity>) -> Self {
        SettingsManager {
            entity_ids
        }
    }
    pub fn disassemble(&mut self, commands: &mut Commands) {
        for entity in &self.entity_ids {
            commands.entity(*entity).despawn();
        }
    }
}
