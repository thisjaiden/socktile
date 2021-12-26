use bevy::{prelude::*, window::WindowMode};
use crate::DEV_BUILD;

pub fn window_setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut windows: ResMut<Windows>
) {
    if DEV_BUILD {
        server.watch_for_changes().unwrap();
    }
    
    let window = windows.get_primary_mut().unwrap();
    window.set_vsync(true);
    window.set_title(String::from("socktile"));
    window.set_maximized(true);
    window.set_mode(WindowMode::BorderlessFullscreen);
    window.set_resolution(1920.0, 1080.0);
    window.set_scale_factor_override(Some(1.0));
    window.set_cursor_visibility(false);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
