use crate::prelude::*;
use std::path::PathBuf;

pub struct Disk {
    window_config: WindowConfig,
    control_config: ControlConfig,
    audio_config: AudioConfig,
    user: Option<User>
}

impl Disk {
    pub fn init() -> Disk {
        #[cfg(target_arch = "wasm32")]
        {
            return Disk {
                window_config: WindowConfig::default(),
                control_config: ControlConfig::default(),
                audio_config: AudioConfig::default(),
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
                    warn!("Encountered courrupted profile data. Resetting data");
                    warn!("Error causing a faliure: {}", att.expect_err("unreachable condition"));
                    user = None;
                }
            }
            else {
                user = None;
            }
            
            let mut window_config_path = files_dir();
            window_config_path.push("window_config.bic");
            let window_config_data = std::fs::read(window_config_path);
            let window_config = if let Ok(data) = window_config_data {
                bincode::deserialize(&data)
                    .expect("Encountered corrupted window configuration data.")
            }
            else {
                WindowConfig::default()
            };

            let mut control_config_path = files_dir();
            control_config_path.push("control_config.bic");
            let control_config_data = std::fs::read(control_config_path);
            let control_config = if let Ok(data) = control_config_data {
                bincode::deserialize(&data)
                    .expect("Encountered corrupted control configuration data.")
            }
            else {
                ControlConfig::default()
            };

            let mut audio_config_path = files_dir();
            audio_config_path.push("audio_config.bic");
            let audio_config_data = std::fs::read(audio_config_path);
            let audio_config = if let Ok(data) = audio_config_data {
                bincode::deserialize(&data)
                    .expect("Encountered corrupted audio configuration data.")
            }
            else {
                AudioConfig::default()
            };

            Disk {
                window_config,
                control_config,
                user,
                audio_config
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
    pub fn control_config(&self) -> ControlConfig {
        self.control_config
    }
    pub fn _update_control_config(&mut self, new: ControlConfig) -> bool {
        let mut control_config_path = files_dir();
        control_config_path.push("control_config.bic");
        let control_config_data = bincode::serialize(&new);
        if let Ok(bytes) = control_config_data {
            if std::fs::write(control_config_path, bytes).is_ok() {
                self.control_config = new;
                return true;
            }
            false
        }
        else {
            false
        }
    }
    pub fn audio_config(&self) -> AudioConfig {
        self.audio_config
    }
    pub fn _update_audio_config(&mut self, new: AudioConfig) -> bool {
        let mut audio_config_path = files_dir();
        audio_config_path.push("audio_config.bic");
        let audio_config_data = bincode::serialize(&new);
        if let Ok(bytes) = audio_config_data {
            if std::fs::write(audio_config_path, bytes).is_ok() {
                self.audio_config = new;
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
            info!("TODO: users are unsaved on wasm");
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

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ControlConfig {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_right: KeyCode,
    pub move_left: KeyCode,
    pub open_chat: KeyCode,
    pub close_menu: KeyCode,
    pub send_chat: KeyCode,
}

impl Default for ControlConfig {
    fn default() -> Self {
        ControlConfig {
            move_up: KeyCode::W,
            move_down: KeyCode::S,
            move_right: KeyCode::D,
            move_left: KeyCode::A,
            open_chat: KeyCode::T,
            close_menu: KeyCode::Escape,
            send_chat: KeyCode::Return
        }
    }
}

fn files_dir() -> PathBuf {
    let mut dir = std::env::current_exe().expect("Unable to get the executable's path.");
    dir.pop();
    dir
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct AudioConfig {
    pub volume: f64
}

impl Default for AudioConfig {
    fn default() -> Self {
        AudioConfig {
            volume: 1.0
        }
    }
}
