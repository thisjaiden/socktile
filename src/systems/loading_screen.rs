use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::DEV_BUILD;
use crate::layers::UI_TEXT;
use crate::resources::AssetHandles;

pub fn loading_screen(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    mut handles: ResMut<AssetHandles>
) {
    if state.eq(&GameState::LoadingScreen) {
        if state.is_added() {
            if DEV_BUILD {
                server.watch_for_changes().unwrap();
            }
            println!("Updating Window...");
            let window = windows.get_primary_mut().unwrap();
            window.set_resizable(false);
            window.set_vsync(true);
            window.set_title(String::from("socktile"));
            window.set_decorations(false);
            window.set_maximized(true);
            let w_factor = 1920.0;
            let h_factor = 1080.0;
            let s_factor = 1.0;
            println!("Width {} Height {} Scale {}", w_factor, h_factor, s_factor);
            //window.set_resolution(w_factor, h_factor);
            //window.set_scale_factor_override(Some(s_factor as f64));
            //window.set_mode(WindowMode::BorderlessFullscreen);
            //window.set_cursor_visibility(false);
            println!("Initalizing camera...");
            commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        }
        let mut path = std::env::current_dir().unwrap();
        path.push("assets");
        let mut wait = false;
        if DEV_BUILD {
            for asset in asset_file_to_list() {
                if asset == String::new() {
                    continue;
                }
                path.push(asset.clone());
                if state.is_added() {
                    let handle: Handle<Texture> = server.load(path.clone());
                    handles.add_texture_handle(handle.clone(), &asset);
                    wait = true;
                }
                else {
                    wait = handles.prod_handle(server.clone());
                }
                path.pop();
            }
        }
        else {
            for asset in ASSET_LIST {
                path.push(asset);
                if state.is_added() {
                    let handle: Handle<Texture> = server.load(path.clone());
                    handles.add_texture_handle(handle.clone(), asset);
                    wait = true;
                }
                else {
                    wait = handles.prod_handle(server.clone());
                }
                path.pop();
            }
        }
        if state.is_added() {
            path.push("base.ttf");
            handles.add_font_handle(server.load(path.clone()), "base.ttf");
            path.pop();
            path.push("KreativeSquare.ttf");
            handles.add_font_handle(server.load(path.clone()), "KreativeSquare.ttf");
            path.pop();
        }
        if !wait {
            println!("Switching state to TitleScreen...");
            state.change_state(GameState::TitleScreen);
        }
        if wait && state.is_added() {
            if DEV_BUILD {
                commands.spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "This is an internal build. Do not distribute.",
                        TextStyle {
                            font: handles.get_font("KreativeSquare.ttf"),
                            font_size: 34.0,
                            color: Color::BLACK
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Top,
                            horizontal: HorizontalAlign::Right
                        }
                    ),
                    transform: Transform::from_xyz(-1920.0 / 2.0, -1080.0 / 2.0, UI_TEXT),
                    ..Default::default()
                });
            }
        }
    }
}

const ASSET_LIST: [&str; 6] = [
    // 1
    "death_noise.png",
    "gblin_exp_2.png",
    "ts.png",
    "green_square.png",
    // 5
    "axe.png",
    "pickaxe.png",
];

fn asset_file_to_list() -> Vec<String> {
    let mut dir = std::env::current_dir().unwrap();
    dir.push("assets.dev");
    let contents = std::fs::read_to_string(dir).unwrap();
    let mut fin = vec![];
    for line in contents.split('\n') {
        fin.push(String::from(line.trim()));
    }
    fin
}
