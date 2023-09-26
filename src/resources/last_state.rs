use bevy::ecs::system::Resource;
use crate::GameState;
use crate::prelude::*;

#[derive(Resource)]
/// Stores the last [GameState] that was active. Used for returning from the
/// settings menu and similar scenarios.
pub struct LastState {
    /// The current [GameState].
    current: Option<GameState>,
    /// The [GameState] before switching to the current one. This might still be
    /// the current [GameState] if you switch to it while in it.
    last: Option<GameState>
}

impl LastState {
    /// Creates a fresh [LastState] struct for inserting as a component.
    pub fn init() -> LastState {
        LastState {
            current: None,
            last: None
        }
    }
    /// Grabs the last [GameState] that occured.
    pub fn get(&self) -> GameState {
        self.last.unwrap()
    }
}

/// Updates the [LastState] resource with the current state, such that it can be
/// copied to the previous state storage when the state changes.
pub fn system_update_last_state_live(
    state: Res<State<GameState>>,
    mut last_state: ResMut<LastState>
) {
    last_state.current = Some(*state.get());
}

/// Updates the [LastState] resource by indicating a state change. Does not take
/// any data in, just shuffles around data internally.
pub fn system_update_last_state(
    mut last_state: ResMut<LastState>
) {
    last_state.last = last_state.current;
}
