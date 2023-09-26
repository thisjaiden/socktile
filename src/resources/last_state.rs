use bevy::ecs::system::Resource;
use crate::GameState;
use crate::prelude::*;

#[derive(Resource)]
pub struct LastState {
    current: Option<GameState>,
    last: Option<GameState>
}

impl LastState {
    pub fn init() -> LastState {
        LastState {
            current: None,
            last: None
        }
    }
    pub fn get(&self) -> GameState {
        self.last.unwrap()
    }
}

pub fn system_update_last_state_live(
    state: Res<State<GameState>>,
    mut last_state: ResMut<LastState>
) {
    last_state.current = Some(*state.get());
}

pub fn system_update_last_state(
    mut last_state: ResMut<LastState>
) {
    last_state.last = last_state.current;
}
