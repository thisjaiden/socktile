use bevy::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextBox {
    buffer: String
}

impl TextBox {
    pub fn init() -> TextBox {
        TextBox {
            buffer: String::new()
        }
    }
    pub fn clear_buffer(&mut self) {
        self.buffer = String::new();
    }
    pub fn grab_buffer(&mut self) -> String {
        self.buffer.clone()
    }
    pub fn update_buffer(&mut self, input: Res<Input<KeyCode>>) {

    }
}
