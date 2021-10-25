use bevy::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetHandles {
    texture_handle: Vec<Handle<Texture>>,
    font_handle: Vec<Handle<Font>>,
    texture_ref_name: Vec<String>,
    font_ref_name: Vec<String>,
    texture_loaded: Vec<bool>,
    font_loaded: Vec<bool>
}

impl AssetHandles {
    pub fn add_texture_handle(&mut self, handle: Handle<Texture>, ref_text: &str) {
        self.texture_handle.push(handle);
        self.texture_ref_name.push(String::from(ref_text));
        self.texture_loaded.push(false);
        println!("Has {} texture handles.", self.texture_handle.len());
    }
    pub fn add_font_handle(&mut self, handle: Handle<Font>, ref_text: &str) {
        self.font_handle.push(handle);
        self.font_ref_name.push(String::from(ref_text));
        self.font_loaded.push(false);
        println!("Has {} font handles.", self.font_handle.len());
    }
    pub fn get_texture(&mut self, ref_text: &str) -> Handle<Texture> {
        let mut location = std::env::current_dir().unwrap();
        location.push("assets");
        location.push(ref_text);
        for i in 0..self.texture_handle.len() {
            if self.texture_ref_name[i] == ref_text {
                return self.texture_handle[i].clone();
            }
        }
        panic!("Texture not found! ({})", ref_text);
    }
    pub fn get_font(&mut self, ref_text: &str) -> Handle<Font> {
        let mut location = std::env::current_dir().unwrap();
        location.push("assets");
        location.push(ref_text);
        for i in 0..self.font_handle.len() {
            if self.font_ref_name[i] == ref_text {
                return self.font_handle[i].clone();
            }
        }
        panic!("Font not found! ({})", ref_text);
    }
    pub fn prod_handle(&mut self, server: AssetServer) -> bool {
        let mut i = 0;
        for handle in self.texture_handle.clone() {
            match server.get_load_state(handle) {
                bevy::asset::LoadState::Loaded => {
                    self.texture_loaded[i] = true;
                }
                _ => {
                    self.texture_loaded[i] = false;
                }
            }
            i += 1;
        }
        let mut i = 0;
        for handle in self.font_handle.clone() {
            match server.get_load_state(handle) {
                bevy::asset::LoadState::Loaded => {
                    //println!("Prodded font is loaded.");
                    self.font_loaded[i] = true;
                }
                _ => {
                    //println!("Prodded font is unloaded.");
                    self.font_loaded[i] = false;
                }
            }
            i += 1;
        }
        let mut waiting_on_load = false;
        i = 0;
        for loadstate in self.texture_loaded.clone() {
            if !loadstate {
                waiting_on_load = true;
            }
            i += 1;
        }
        i = 0;
        for loadstate in self.font_loaded.clone() {
            if !loadstate {
                waiting_on_load = true;
            }
            i += 1;
        }
        return waiting_on_load;
    }
    pub fn init() -> Self {
        Self {
            texture_handle: vec![],
            font_handle: vec![],
            texture_ref_name: vec![],
            font_ref_name: vec![],
            texture_loaded: vec![],
            font_loaded: vec![]
        }
    }
}
