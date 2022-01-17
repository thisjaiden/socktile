use bevy::prelude::*;
use bevy_asset_loader::{AssetLoader, AssetCollection};

mod components;
mod systems;
mod resources;
mod layers;
mod server;
mod client;
mod shared;
mod window_setup;
mod ldtk;

// Is this an internal dev build?
pub const DEV_BUILD: bool = true;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    /// Loads assets from disk
    Load,
    /// Checks network status
    NetworkCheck,
    /// Offline mode title screen
    OfflineTitle,
    /// Online mode title screen
    TitleScreen,
    /// User creation screen
    MakeUser,
    /// Server listings
    ServerList,
    /// World creation screen
    MakeGame,
    /// Settings screen
    Settings,
    /// Gameplay state
    Play,
}

fn main() {
    if DEV_BUILD {
        // Allow GGS to be run if it's a dev build, and warn about distribution.
        println!("\x1B[40;91mWARNING: This is an internal build. All software is property of and (c) Jaiden Bernard. Do not share this software without permission from the property owners.\x1B[0m");
        println!("Sidenote: if you just built this from GitHub, do as you will. This doesn't apply to you.");
        let mut args = std::env::args();
        args.next();
        if let Some(argument) = args.next() {
            if argument == "--ggs" {
                println!("\x1B[40;91mWARNING: Running as a GGS. Make sure you know what you're doing!\x1B[0m");
                server::core::startup();
            }
        }
    }

    // Create our Bevy app!
    let mut app = App::new();
    // Register all the assets we need loaded.
    AssetLoader::new(GameState::Load)
        .continue_to_state(GameState::NetworkCheck)
        .with_collection::<MapAssets>()
        .with_collection::<FontAssets>()
        .with_collection::<AnimatorAssets>()
        .build(&mut app);
    // Add plugins and systems to our app, then run it!
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(benimator::AnimationPlugin)
        .add_plugin(bevy_kira_audio::AudioPlugin)
        .add_plugin(ldtk::LDtkPlugin)
        .add_state(GameState::Load)
        .add_system_set(
            SystemSet::on_enter(GameState::Load)
                .with_system(window_setup::window_setup)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::NetworkCheck)
                .with_system(systems::netty::startup_checks)
                .with_system(systems::cursor::spawn)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::TitleScreen)
                .with_system(systems::visual::load_title_screen_map)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::OfflineTitle)
                .with_system(systems::visual::load_offline_title_map)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MakeUser)
                .with_system(systems::visual::load_user_creation_map)
        )
        .add_system_set(
            SystemSet::on_update(GameState::MakeUser)
                .with_system(systems::text_box::user_creation)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MakeGame)
                .with_system(systems::text_box::game_creation_once)
        )
        .add_system_set(
            SystemSet::on_update(GameState::MakeGame)
                .with_system(systems::text_box::game_creation)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::ServerList)
                .with_system(systems::netty::server_list)
        )
        .add_system_set(
            SystemSet::on_update(GameState::ServerList)
                .with_system(resources::Reality::system_server_list_renderer)
        )
        .add_system(systems::cursor::cursor)
        .insert_resource(resources::TextBox::init())
        .add_system(systems::text_box::text_box)
        .insert_resource(resources::Netty::init())
        .add_system(systems::netty::step)
        .insert_resource(resources::ui::UIManager::init())
        .add_system(resources::ui::ui_scene)
        .add_system(resources::ui::ui_game)
        .add_system(resources::ui::ui_manager)
        .add_system(resources::ui::ui_quick_exit)
        .insert_resource(resources::Reality::init())
        .insert_resource(resources::Animator::init())
        .add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(resources::Reality::system_chunk_loader)
                .with_system(resources::Reality::system_player_loader)
                .with_system(resources::Reality::system_player_controls)
                .with_system(resources::Reality::system_camera_updater)
                .with_system(resources::Reality::system_player_locator)
                .with_system(resources::Animator::system_player_animator)
        )
        .run();
}

// Below are the assets used in this application.
// TODO: These should probably be moved to assets.rs or something.

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "core.ldtk")]
    player: Handle<ldtk::LDtkMap>,
}

#[derive(AssetCollection, Clone)]
pub struct FontAssets {
    #[asset(path = "font/apple_tea.ttf")]
    _apple_tea: Handle<Font>,
    #[asset(path = "font/simvoni/regular.ttf")]
    simvoni: Handle<Font>,
    #[asset(path = "font/simvoni/italic.ttf")]
    _simvoni_italic: Handle<Font>,
    #[asset(path = "font/simvoni/bold.ttf")]
    _simvoni_bold: Handle<Font>,
    #[asset(path = "font/simvoni/bolditalic.ttf")]
    _simvoni_bold_italic: Handle<Font>,
    /// WARNING: DEPRECATED FONT
    #[asset(path = "font/kreative_square.ttf")]
    kreative_square: Handle<Font>
}

#[derive(AssetCollection)]
pub struct AnimatorAssets {
    #[asset(path = "player/placeholder.png")]
    placeholder: Handle<Image>
}
