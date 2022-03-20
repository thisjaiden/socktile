use bevy::prelude::*;

use super::Reality;

pub struct Chat {
    pub history: Vec<ChatMessage>
}

impl Chat {
    pub fn init() -> Chat {
        Chat {
            history: vec![]
        }
    }
    fn add_message(&mut self, msg: ChatMessage) {
        self.history.push(msg);
        // todo: cleanse pool if needed
    }
    pub fn system_display_chat(
    ) {

    }
    pub fn system_pull_messages(
        mut selfs: ResMut<Chat>,
        mut reality: ResMut<Reality>
    ) {
        for message in reality.pull_messages() {
            selfs.add_message(message);
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ChatMessage {
    pub text: String,
    pub color: Color,
    pub sent_at: std::time::Instant
}
