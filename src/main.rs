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
          .with_collection::<AnimatorAssets>()
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
        .add_system_set(
            SystemSet::on_enter(GameState::OfflineTitle)
                .with_system(systems::visual::load_offline_title_map.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MakeUser)
                .with_system(systems::visual::load_user_creation_map.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::MakeUser)
                .with_system(systems::text_box::user_creation.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MakeGame)
                .with_system(systems::text_box::game_creation_once.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::MakeGame)
                .with_system(systems::text_box::game_creation.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::ServerList)
                .with_system(systems::netty::server_list.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::ServerList)
                .with_system(resources::Reality::system_server_list_renderer.system())
        )
        .add_system(systems::cursor::cursor.system())
        .insert_resource(resources::TextBox::init())
        .add_system(systems::text_box::text_box.system())
        .insert_resource(resources::Netty::init())
        .add_system(systems::netty::step.system())
        .insert_resource(resources::ui::UIManager::init())
        .add_system(resources::ui::ui_scene.system())
        .add_system(resources::ui::ui_game.system())
        .add_system(resources::ui::ui_manager.system())
        .insert_resource(resources::Reality::init())
        .insert_resource(resources::Animator::init())
        .add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(resources::Reality::system_chunk_loader.system())
                .with_system(resources::Reality::system_player_controls.system())
                .with_system(resources::Reality::system_camera_updater.system())
                .with_system(resources::Reality::system_player_locator.system())
                .with_system(resources::Animator::system_player_animator.system())
        )
        .run();
}

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "core.ldtk")]
    player: Handle<ldtk::LDtkMap>,
}

#[derive(AssetCollection, Clone)]
pub struct FontAssets {
    #[asset(path = "font/apple_tea.ttf")]
    apple_tea: Handle<Font>,
    #[asset(path = "font/simvoni/regular.ttf")]
    simvoni: Handle<Font>,
    #[asset(path = "font/simvoni/italic.ttf")]
    simvoni_italic: Handle<Font>,
    #[asset(path = "font/simvoni/bold.ttf")]
    simvoni_bold: Handle<Font>,
    #[asset(path = "font/simvoni/bolditalic.ttf")]
    simvoni_bold_italic: Handle<Font>,
    /// WARNING: DEPRECATED FONT
    #[asset(path = "font/kreative_square.ttf")]
    kreative_square: Handle<Font>
}

#[derive(AssetCollection)]
pub struct AnimatorAssets {
    #[asset(color_material)]
    #[asset(path = "player/placeholder.png")]
    placeholder: Handle<ColorMaterial>
}
