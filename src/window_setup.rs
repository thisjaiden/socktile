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
    println!("Updating Window...");
    let window = windows.get_primary_mut().unwrap();
    window.set_resizable(false);
    window.set_vsync(true);
    window.set_title(String::from("socktile"));
    // window.set_decorations(false);
    window.set_maximized(true);
    window.set_mode(WindowMode::BorderlessFullscreen);
    let w_factor = 1920.0;
    let h_factor = 1080.0;
    let s_factor = 1.0;
    println!("Width {} Height {} Scale {}", w_factor, h_factor, s_factor);
    window.set_resolution(w_factor, h_factor);
    window.set_scale_factor_override(Some(s_factor as f64));
    
    // window.set_cursor_visibility(false);
    println!("Initalizing camera...");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}