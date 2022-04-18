use bevy::{prelude::*, window::PresentMode};
use crate::resources::Disk;

pub fn window_setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    conf: Res<Disk>
) { 
    let window = windows.get_primary_mut().unwrap();
    let window_conf = conf.window_config();

    if window_conf.vsync {
        window.set_present_mode(PresentMode::Fifo);
    }
    else {
        window.set_present_mode(PresentMode::Mailbox);
    }
    
    window.set_title(String::from("socktile"));
    if window_conf.fullscreen {
        window.set_mode(bevy::window::WindowMode::BorderlessFullscreen);
    }
    window.set_resolution(window_conf.resolution.0, window_conf.resolution.1);
    window.set_scale_factor_override(Some(window_conf.scale_factor));
    window.set_cursor_visibility(false);
    
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
