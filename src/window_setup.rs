use bevy::window::{PrimaryWindow, WindowMode, PresentMode};

use crate::prelude::*;

pub fn window_setup(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    conf: Res<Disk>
) {
    let mut window = window_query.get_single_mut().unwrap();
    let window_conf = conf.window_config();

    if window_conf.vsync {
        window.present_mode = PresentMode::AutoVsync;
    }
    else {
        window.present_mode = PresentMode::AutoNoVsync;
    }

    window.title = String::from("socktile");
    if window_conf.fullscreen {
        window.resolution.set(window_conf.resolution.0, window_conf.resolution.1);
        window.mode = WindowMode::SizedFullscreen;
    }
    else {
        window.resolution.set(window_conf.resolution.0, window_conf.resolution.1);
        window.resolution.set_scale_factor_override(Some(window_conf.scale_factor));
        window.mode = WindowMode::Windowed;
    }

    window.resolution.set_scale_factor_override(Some(window_conf.scale_factor));
    window.cursor.visible = false;

    commands.spawn(Camera2dBundle::default());
}

pub fn window_update(mut window_query: Query<&mut Window, With<PrimaryWindow>>, conf: Res<Disk>) {
    if conf.is_changed() {
        let mut window = window_query.get_single_mut().unwrap();
        let window_conf = conf.window_config();

        if window_conf.vsync {
            window.present_mode = PresentMode::AutoVsync;
        }
        else {
            window.present_mode = PresentMode::AutoNoVsync;
        }

        if window_conf.fullscreen {
            window.mode = WindowMode::SizedFullscreen;
        }
        else {
            window.resolution.set(window_conf.resolution.0, window_conf.resolution.1);
            window.resolution.set_scale_factor_override(Some(window_conf.scale_factor));
            window.mode = WindowMode::Windowed;
        }
    }
}
