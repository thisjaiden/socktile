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
    pub fn eat_buffer(&mut self) {
        // take characters
        let mut chars = self.buffer.chars();
        // discard last
        chars.next_back();
        // reassign data
        self.buffer = String::from(chars.as_str());
    }
    pub fn update_buffer(&mut self, input: char) {
        self.buffer += &input.to_string();
    }
}
