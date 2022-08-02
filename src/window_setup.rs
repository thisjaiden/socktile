use crate::prelude::*;

pub fn window_setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    conf: Res<Disk>
) { 
    let window = windows.get_primary_mut().unwrap();
    let window_conf = conf.window_config();

    if window_conf.vsync {
        window.set_present_mode(bevy::window::PresentMode::Fifo);
    }
    else {
        window.set_present_mode(bevy::window::PresentMode::Immediate);
    }
    
    window.set_title(String::from("socktile"));
    if window_conf.fullscreen {
        window.set_resolution(window_conf.resolution.0, window_conf.resolution.1);
        window.set_mode(bevy::window::WindowMode::SizedFullscreen);
    }
    else {
        window.set_resolution(window_conf.resolution.0, window_conf.resolution.1);
        window.set_scale_factor_override(Some(window_conf.scale_factor));
        window.set_mode(bevy::window::WindowMode::Windowed);
    }
    
    window.set_scale_factor_override(Some(window_conf.scale_factor));
    window.set_cursor_visibility(false);
    
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn window_update(
    mut windows: ResMut<Windows>,
    conf: Res<Disk>
) {
    if conf.is_changed() {
        let window = windows.get_primary_mut().unwrap();
        let window_conf = conf.window_config();

        if window_conf.vsync {
            window.set_present_mode(bevy::window::PresentMode::Fifo);
        }
        else {
            window.set_present_mode(bevy::window::PresentMode::Immediate);
        }
        
        if window_conf.fullscreen {
            window.set_mode(bevy::window::WindowMode::SizedFullscreen);
        }
        else {
            window.set_resolution(window_conf.resolution.0, window_conf.resolution.1);
            window.set_scale_factor_override(Some(window_conf.scale_factor));
            window.set_mode(bevy::window::WindowMode::Windowed);
        }
    }
}
