#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameState {
    LoadingScreen,
    TitleScreen,
    Settings,
    Join,
    CreateUser,
    CreateUserB,
    New,
    Play
}

impl GameState {
    pub fn change_state(&mut self, state: GameState) {
        *self = state;
    }
}
