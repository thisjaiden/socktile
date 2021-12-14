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

// Build switches
// --------------
// Is this an internal dev build?
pub const DEV_BUILD: bool      = true;
// Should UI debug lines be shown?
pub const DEBUG_UI: bool       = true;
// Should hitbox debug lines be shown?
pub const DEBUG_HITBOXES: bool = false;

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
        println!("\x1B[40;91mTHIS IS AN INTERNAL BUILD. DO NOT DISTRIBUTE.\x1B[0m");
        let mut args = std::env::args();
        args.next();
        if let Some(argument) = args.next() {
            if argument == "--ggs" {
                println!("\x1B[40;91mRUNNING AS A GLOBAL GAME SERVER. DO NOT RUN FROM THE WRONG LOCATION. DO NOT HAVE MULTIPLE INSTANCES RUNNING.\x1B[0m");
                server::core::startup();
            }
        }
    }
    let mut app = App::build();
    AssetLoader::new(GameState::Load, GameState::NetworkCheck)
          .with_collection::<MapAssets>()
          .with_collection::<FontAssets>()
          .build(&mut app);
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(benimator::AnimationPlugin)
        .add_plugin(bevy_kira_audio::AudioPlugin)
        .add_plugin(ldtk::LDtkPlugin)
        .add_state(GameState::Load)
        .add_system_set(
            SystemSet::on_enter(GameState::Load)
                .with_system(window_setup::window_setup.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::NetworkCheck)
                .with_system(systems::netty::startup_checks.system())
                .with_system(systems::cursor::spawn.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::TitleScreen)
                .with_system(systems::visual::load_title_screen_map.system())
        )
        .add_system(systems::cursor::cursor.system())
        .insert_resource(resources::TextBox::init())
        .add_system(systems::text_box::text_box.system())
        .insert_resource(resources::Netty::init())
        .add_system(systems::netty::step.system())
        .insert_resource(resources::Reality::init())
        .run();
}

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "core.ldtk")]
    player: Handle<ldtk::LDtkMap>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "font/base.ttf")]
    base: Handle<Font>,
    #[asset(path = "font/KreativeSquare.ttf")]
    kreative_square: Handle<Font>,
}
