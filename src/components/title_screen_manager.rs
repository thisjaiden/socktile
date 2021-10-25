use bevy::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TitleScreenManager {
    entity_ids: Vec<Entity>
}

impl TitleScreenManager {
    pub fn new(entity_ids: Vec<Entity>) -> Self {
        TitleScreenManager {
            entity_ids
        }
    }
    pub fn disassemble(&mut self, commands: &mut Commands) {
        for entity in &self.entity_ids {
            commands.entity(*entity).despawn();
        }
    }
}
