use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::shared::saves::User;

pub struct Disk {
    window_config: WindowConfig,
    user: Option<User>
}

impl Disk {
    pub fn init() -> Disk {
        #[cfg(target_arch = "wasm32")]
        {
            return Disk {
                window_config: WindowConfig::default(),
                user: None
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut user_path = files_dir();
            user_path.push("user_profile.bic");
            let user_data = std::fs::read(user_path);
            let user: Option<User>;
            if let Ok(data) = user_data {
                let att = bincode::deserialize(&data);
                if let Ok(desered) = att {
                    user = Some(desered);
                }
                else {
                    println!("WARNING: Encountered courrupted profile data. Resetting data.");
                    println!("Error causing a faliure: {}", att.expect_err("unreachable condition"));
                    user = None;
                }
            }
            else {
                user = None;
            }
            
            let mut window_config_path = files_dir();
            window_config_path.push("window_config.bic");
            let window_config_data = std::fs::read(window_config_path);
            let window_config: WindowConfig;
            if let Ok(data) = window_config_data {
                window_config = bincode::deserialize(&data)
                    .expect("Encountered corrupted window configuration data.");
            }
            else {
                window_config = WindowConfig::default();
            }

            return Disk {
                window_config,
                user
            }
        }
    }
    pub fn window_config(&self) -> WindowConfig {
        self.window_config
    }
    pub fn update_window_config(&mut self, new: WindowConfig) -> bool {
        let mut window_config_path = files_dir();
        window_config_path.push("window_config.bic");
        let window_config_data = bincode::serialize(&new);
        if let Ok(bytes) = window_config_data {
            if std::fs::write(window_config_path, bytes).is_ok() {
                self.window_config = new;
                return true;
            }
            false
        }
        else {
            false
        }
    }
    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }
    pub fn update_user(&mut self, new: User) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            println!("TODO: users are unsaved on wasm");
            self.user = Some(new);
            true
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut user_path = files_dir();
            user_path.push("user_profile.bic");
            let user_data = bincode::serialize(&new);
            if let Ok(bytes) = user_data {
                if std::fs::write(user_path, bytes).is_ok() {
                    self.user = Some(new);
                    return true;
                }
                false
            }
            else {
                false
            }
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WindowConfig {
    pub vsync: bool,
    pub fullscreen: bool,
    pub resolution: (f32, f32),
    pub scale_factor: f64
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            vsync: true,
            fullscreen: false,
            resolution: (1920.0, 1080.0),
            scale_factor: 1.0
        }
    }
}

fn files_dir() -> PathBuf {
    let mut dir = std::env::current_exe().expect("Unable to get the executable's path.");
    dir.pop();
    dir
}
