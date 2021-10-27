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
        let mut is_caps = input.pressed(KeyCode::LShift);
        if !is_caps {
            is_caps = input.pressed(KeyCode::RShift);
        }
        if input.just_pressed(KeyCode::Apostrophe) {
            if is_caps {
                self.buffer += "~";
            }
            else {
                self.buffer += "`";
            }
        }
        if input.just_pressed(KeyCode::Key1) {
            if is_caps {
                self.buffer += "!";
            }
            else {
                self.buffer += "1";
            }
        }
        if input.just_pressed(KeyCode::Key2) {
            if is_caps {
                self.buffer += "@";
            }
            else {
                self.buffer += "2";
            }
        }
        if input.just_pressed(KeyCode::Key3) {
            if is_caps {
                self.buffer += "#";
            }
            else {
                self.buffer += "3";
            }
        }
        if input.just_pressed(KeyCode::Key4) {
            if is_caps {
                self.buffer += "$";
            }
            else {
                self.buffer += "4";
            }
        }
        if input.just_pressed(KeyCode::Key5) {
            if is_caps {
                self.buffer += "%";
            }
            else {
                self.buffer += "5";
            }
        }
        if input.just_pressed(KeyCode::Key6) {
            if is_caps {
                self.buffer += "^";
            }
            else {
                self.buffer += "6";
            }
        }
        if input.just_pressed(KeyCode::Key7) {
            if is_caps {
                self.buffer += "&";
            }
            else {
                self.buffer += "7";
            }
        }
        if input.just_pressed(KeyCode::Key8) {
            if is_caps {
                self.buffer += "*";
            }
            else {
                self.buffer += "8";
            }
        }
        if input.just_pressed(KeyCode::Key9) {
            if is_caps {
                self.buffer += "(";
            }
            else {
                self.buffer += "9";
            }
        }
        if input.just_pressed(KeyCode::Key0) {
            if is_caps {
                self.buffer += ")";
            }
            else {
                self.buffer += "0";
            }
        }
        if input.just_pressed(KeyCode::Minus) {
            if is_caps {
                self.buffer += "_";
            }
            else {
                self.buffer += "-";
            }
        }
        if input.just_pressed(KeyCode::Equals) {
            if is_caps {
                self.buffer += "+";
            }
            else {
                self.buffer += "=";
            }
        }
        if input.just_pressed(KeyCode::Back) {
            if !self.buffer.is_empty() {
                self.buffer.remove(self.buffer.len());
            }
        }
        if input.just_pressed(KeyCode::Q) {
            if is_caps {
                self.buffer += "Q";
            }
            else {
                self.buffer += "q";
            }
        }
        if input.just_pressed(KeyCode::W) {
            if is_caps {
                self.buffer += "W";
            }
            else {
                self.buffer += "w";
            }
        }
        if input.just_pressed(KeyCode::E) {
            if is_caps {
                self.buffer += "E";
            }
            else {
                self.buffer += "e";
            }
        }
        if input.just_pressed(KeyCode::R) {
            if is_caps {
                self.buffer += "R";
            }
            else {
                self.buffer += "r";
            }
        }
        if input.just_pressed(KeyCode::T) {
            if is_caps {
                self.buffer += "T";
            }
            else {
                self.buffer += "t";
            }
        }
        if input.just_pressed(KeyCode::Y) {
            if is_caps {
                self.buffer += "Y";
            }
            else {
                self.buffer += "y";
            }
        }
        if input.just_pressed(KeyCode::U) {
            if is_caps {
                self.buffer += "U";
            }
            else {
                self.buffer += "u";
            }
        }
        if input.just_pressed(KeyCode::I) {
            if is_caps {
                self.buffer += "I";
            }
            else {
                self.buffer += "i";
            }
        }
        if input.just_pressed(KeyCode::O) {
            if is_caps {
                self.buffer += "O";
            }
            else {
                self.buffer += "o";
            }
        }
        if input.just_pressed(KeyCode::P) {
            if is_caps {
                self.buffer += "P";
            }
            else {
                self.buffer += "p";
            }
        }
        if input.just_pressed(KeyCode::LBracket) {
            if is_caps {
                self.buffer += "{";
            }
            else {
                self.buffer += "[";
            }
        }
        if input.just_pressed(KeyCode::RBracket) {
            if is_caps {
                self.buffer += "}";
            }
            else {
                self.buffer += "]";
            }
        }
        if input.just_pressed(KeyCode::Backslash) {
            if is_caps {
                self.buffer += "|";
            }
            else {
                self.buffer += "\\";
            }
        }
        if input.just_pressed(KeyCode::A) {
            if is_caps {
                self.buffer += "A";
            }
            else {
                self.buffer += "a";
            }
        }
        if input.just_pressed(KeyCode::S) {
            if is_caps {
                self.buffer += "S";
            }
            else {
                self.buffer += "s";
            }
        }
        if input.just_pressed(KeyCode::D) {
            if is_caps {
                self.buffer += "D";
            }
            else {
                self.buffer += "d";
            }
        }
        if input.just_pressed(KeyCode::F) {
            if is_caps {
                self.buffer += "F";
            }
            else {
                self.buffer += "f";
            }
        }
        if input.just_pressed(KeyCode::G) {
            if is_caps {
                self.buffer += "G";
            }
            else {
                self.buffer += "g";
            }
        }
        if input.just_pressed(KeyCode::H) {
            if is_caps {
                self.buffer += "H";
            }
            else {
                self.buffer += "h";
            }
        }
    }
}
