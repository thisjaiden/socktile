#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use consts::EMBED_ASSETS;
use tracing_subscriber::{prelude::*, EnvFilter};

mod components;
mod systems;
mod resources;
mod consts;
mod server;
mod shared;
mod window_setup;
mod ldtk;
mod assets;

/// Represents the state the game is currently in. Used to keep track of what systems to run.
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
    // Set up the logger. We do this without bevy because the server doesn't set up a bevy app.
    log_setup();
    
    // Warn about distribution of internal builds
    if consts::DEV_BUILD {
        info!("This is an internal build. All software is property of and (c) Jaiden Bernard 2021-2022.");
        info!("Do not share this software without permission from the property owners.");
    }
    // If starting a server is allowed...
    if consts::ALLOW_GGS {
        // Grab CLI arguments
        let mut args = std::env::args();
        // Throw away the caller path, we don't need it
        args.next();
        // Collect the rest of the arguments
        let arguments: Vec<String> = args.collect();
        // If one of the arguments is `server`...
        if arguments.contains(&String::from("server")) {
            // Run a server
            // `server::startup();` returns a never type and should never proceed to launching a normal game.
            info!("Running as a server. Make sure you know what you're doing!");
            server::startup(arguments);
        }
    }

    // Create our Bevy app!
    let mut app = App::new();

    // Enable embedded assets through `bevy_embedded_assets`
    app.add_plugins_with(DefaultPlugins, |group| {
        if EMBED_ASSETS {
            group
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
                .disable::<bevy::log::LogPlugin>()
            }
        else {
            group
                .disable::<bevy::log::LogPlugin>()
        }
    });
    // Register all the assets we need loaded
    AssetLoader::new(GameState::Load)
        .continue_to_state(GameState::NetworkCheck)
        .with_collection::<assets::MapAssets>()
        .with_collection::<assets::FontAssets>()
        .with_collection::<assets::AnimatorAssets>()
        .with_collection::<assets::UIAssets>()
        .with_collection::<assets::ItemAssets>()
        .with_collection::<assets::ObjectAssets>()
        .with_collection::<assets::NPCAssets>()
        .with_collection::<assets::AudioAssets>()
        .build(&mut app);
    
    // Add plugins and systems to our app, then run it
    app
        .add_plugin(ldtk::LDtkPlugin)
        .add_plugin(bevy_kira_audio::AudioPlugin)
        .add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::default())
        .add_state(GameState::Load)
        .add_system_set(
            SystemSet::on_enter(GameState::Load)
                .with_system(window_setup::window_setup)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::NetworkCheck)
                .with_system(resources::Netty::system_startup_checks)
                .with_system(systems::cursor::spawn)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::TitleScreen)
                .with_system(systems::visual::load_title_screen_map)
                .with_system(systems::audio::title_screen_loop)
        )
        .add_system_set(
            SystemSet::on_resume(GameState::TitleScreen)
                .with_system(systems::visual::load_title_screen_map)
        )
        .add_system_set(
            SystemSet::on_resume(GameState::Play)
                .with_system(resources::ui::ui_resume_game_settings)
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
            SystemSet::on_enter(GameState::Settings)
                .with_system(systems::visual::load_settings_map)
                .with_system(resources::ui::ui_settings_camera)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Settings)
                .with_system(resources::ui::ui_settings_page)
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
                .with_system(resources::Netty::system_server_list)
        )
        .add_system_set(
            SystemSet::on_update(GameState::ServerList)
                .with_system(resources::Reality::system_server_list_renderer)
        )
        .add_system(window_setup::window_update)
        .add_system(systems::cursor::cursor.label("cursor"))
        .add_system(systems::text_box::text_input)
        .add_system(systems::text_box::text_backspace)
        .add_system(resources::Netty::system_step)
        .add_system(resources::ui::ui_scene)
        .add_system(resources::ui::ui_game)
        .add_system(resources::ui::ui_manager.after("cursor").before("player"))
        .add_system(resources::ui::ui_quick_exit)
        .add_system(resources::ui::ui_close_pause_menu)
        .add_system(resources::ui::ui_disconnect_game)
        .add_system(resources::ui::ui_invite_menu)
        .add_system(resources::ui::ui_close_settings)
        .add_system(resources::ui::ui_debug_lines)
        .add_system(resources::ui::ui_settings_text_updater)
        .insert_resource(resources::Reality::init())
        .insert_resource(resources::Animator::init())
        .insert_resource(resources::TextBox::init())
        .insert_resource(resources::Netty::init())
        .insert_resource(resources::ui::UIManager::init())
        .insert_resource(resources::Disk::init())
        .insert_resource(resources::Chat::init())
        .add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(resources::Reality::system_spawn_objects)
                .with_system(resources::Reality::system_pause_menu)
                .with_system(resources::Reality::system_chunk_loader)
                .with_system(resources::Reality::system_chunk_unloader)
                .with_system(resources::Reality::system_player_loader)
                .with_system(resources::Reality::system_player_unloader)
                .with_system(resources::Reality::system_player_controls)
                .with_system(resources::Reality::system_camera_updater.label("ui").after("player"))
                .with_system(resources::Reality::system_player_locator.label("player"))
                .with_system(resources::Reality::system_pause_renderer.before("ui"))
                .with_system(resources::Reality::system_position_hotbar.before("ui"))
                .with_system(resources::Reality::system_scroll_hotbar)
                .with_system(resources::Reality::system_pause_invite)
                .with_system(resources::Reality::system_update_objects)
                .with_system(resources::Reality::system_remove_objects)
                .with_system(resources::Reality::system_update_hotbar)
                .with_system(resources::Reality::system_render_waiting_chunks)
                .with_system(resources::Animator::system_player_animator)
                .with_system(resources::Animator::system_player_initiator)
                .with_system(resources::Chat::system_display_chat)
                .with_system(resources::Chat::system_pull_messages)
                .with_system(resources::Chat::system_open_chat)
                .with_system(resources::Chat::system_type_chat)
                .with_system(resources::Chat::system_send_chat)
                .with_system(resources::ui::ui_forward)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Play)
                .with_system(resources::Reality::system_spawn_hotbar)
                .with_system(resources::Chat::system_init)
        )
        .add_system_set(
            SystemSet::on_update(GameState::TitleScreen)
                .with_system(systems::visual::update_title_screen_user)
                .with_system(systems::visual::update_title_screen_camera)
        )
        .run();
}

fn log_setup() {
    tracing_log::LogTracer::init().unwrap();
    let fmt_layer;
    if consts::DEV_BUILD {
        fmt_layer = tracing_subscriber::fmt::Layer::default()
            .without_time()
            .with_file(false)
            .with_line_number(true);
    }
    else {
        fmt_layer = tracing_subscriber::fmt::Layer::default()
            .without_time()
            .with_file(false)
            .with_line_number(false);
    }
    let subscriber = tracing_subscriber::Registry::default()
        .with(fmt_layer)
        .with(EnvFilter::new("INFO,wgpu=error,symphonia=error"));
    tracing::subscriber::set_global_default(subscriber)
        .expect("Couldn't set global tracing subscriber");
}
