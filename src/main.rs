#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use crate::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use tracing_subscriber::{prelude::*, EnvFilter};
use iyes_progress::ProgressPlugin;

mod components;
mod systems;
mod resources;
mod consts;
mod server;
mod shared;
mod window_setup;
mod modular_assets;
mod assets;
mod matrix;
mod prelude;

/// Represents the state the game is currently in. Used to keep track of what systems to run.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    /// Loads logo from disk and continues to `Load`
    PreLoadLoad,
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
    
    // Add assets, plugins and systems to our app, then run it
    app
        .add_loading_state(
            LoadingState::new(GameState::Load)
                .with_collection::<assets::CoreAssets>()
                .with_collection::<assets::FontAssets>()
                .with_collection::<assets::AnimatorAssets>()
                .with_collection::<assets::UIAssets>()
                .with_collection::<assets::ItemAssets>()
                .with_collection::<assets::ObjectAssets>()
                .with_collection::<assets::NPCAssets>()
        )
        .add_plugin(ProgressPlugin::new(GameState::Load).continue_to(GameState::NetworkCheck))
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugin(modular_assets::ModularAssetsPlugin)
        .add_plugin(bevy_kira_audio::AudioPlugin)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::default())
        .add_state(GameState::PreLoadLoad)
        .add_system_set(
            SystemSet::on_enter(GameState::PreLoadLoad)
                .with_system(systems::visual::logo)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Load)
                .with_system(window_setup::window_setup)
                .with_system(systems::audio::audio_setup)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Load)
                .with_system(systems::visual::loading_prog)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::NetworkCheck)
                .with_system(resources::network::system_startup_checks)
                .with_system(systems::cursor::spawn)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::TitleScreen)
                .with_system(systems::visual::title_screen.label("any"))
                .with_system(systems::audio::title_screen_loop.label("any"))
                .with_system(systems::visual::clear_old.before("any"))
        )
        .add_system_set(
            SystemSet::on_resume(GameState::TitleScreen)
                .with_system(systems::visual::clear_settings)
                .with_system(systems::visual::title_screen)
        )
        .add_system_set(
            SystemSet::on_resume(GameState::Play)
                .with_system(resources::ui::ui_resume_game_settings)
                .with_system(systems::visual::clear_settings)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::OfflineTitle)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MakeUser)
                .with_system(systems::visual::make_user.label("any"))
                .with_system(systems::visual::clear_old.before("any"))
        )
        .add_system_set(
            SystemSet::on_update(GameState::MakeUser)
                .with_system(systems::text_box::user_creation)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Settings)
                .with_system(systems::visual::settings_video)
                .with_system(resources::ui::ui_settings_camera)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Settings)
                .with_system(resources::ui::ui_settings_tab)
                .with_system(resources::ui::ui_toggle_fullscreen)
                .with_system(resources::ui::ui_return_titlescreen)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MakeGame)
                .with_system(systems::visual::clear_old.before("any"))
                .with_system(systems::text_box::game_creation_once.label("any"))
                .with_system(systems::visual::create_world.label("any"))
        )
        .add_system_set(
            SystemSet::on_update(GameState::MakeGame)
                .with_system(systems::text_box::game_creation)
                .with_system(resources::ui::ui_return_titlescreen)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::ServerList)
                .with_system(resources::network::system_server_list.label("any"))
                .with_system(systems::visual::join_world.label("any"))
                .with_system(systems::visual::clear_old.before("any"))
        )
        .add_system_set(
            SystemSet::on_update(GameState::ServerList)
                .with_system(resources::Reality::system_server_list_renderer)
                .with_system(resources::ui::ui_game)
                .with_system(resources::ui::ui_return_titlescreen)
        )
        .add_system(window_setup::window_update)
        .add_system(systems::cursor::cursor.label("cursor"))
        .add_system(systems::text_box::text_input)
        .add_system(resources::network::system_step)
        .add_system(resources::ui::ui_open_settings)
        .add_system(resources::ui::ui_manager.after("cursor").before("player"))
        .add_system(resources::ui::ui_quick_exit)
        .add_system(resources::ui::ui_close_pause_menu)
        .add_system(resources::ui::ui_invite_menu)
        .add_system(resources::ui::ui_close_settings)
        .add_system(resources::ui::ui_debug_lines)
        .add_system(resources::ui::ui_settings_text_updater)
        .insert_resource(resources::Reality::init())
        .insert_resource(resources::Animator::init())
        .insert_resource(resources::TextBox::init())
        .insert_resource(resources::network::init())
        .insert_resource(resources::ui::UIManager::init())
        .insert_resource(resources::Disk::init())
        .insert_resource(resources::Chat::init())
        .add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(resources::Reality::system_spawn_objects)
                .with_system(resources::Reality::system_pause_menu)
                .with_system(resources::Reality::system_chunk_requester)
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
                .with_system(resources::Reality::system_hitbox_debug_lines)
                .with_system(resources::Reality::system_player_debug_lines)
                .with_system(resources::Reality::system_chunk_derenderer)
                .with_system(resources::Reality::system_rerender_edges)
                .with_system(resources::Reality::system_render_waiting_chunks)
                .with_system(resources::Reality::system_action_none)
                .with_system(resources::Reality::system_action_chop)
                .with_system(resources::Animator::system_player_animator)
                .with_system(resources::Animator::system_player_initiator)
                .with_system(resources::Chat::system_display_chat)
                .with_system(resources::Chat::system_pull_messages)
                .with_system(resources::Chat::system_open_chat)
                .with_system(resources::Chat::system_type_chat)
                .with_system(resources::Chat::system_send_chat)
                .with_system(resources::ui::ui_forward)
                .with_system(resources::ui::ui_disconnect_game)
                .with_system(resources::Reality::system_mark_chunks)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Play)
                .with_system(resources::Reality::system_spawn_hotbar.label("any"))
                .with_system(resources::Chat::system_init.label("any"))
                .with_system(systems::visual::clear_old.before("any"))
        )
        .add_system_set(
            SystemSet::on_update(GameState::TitleScreen)
                .with_system(systems::visual::update_title_screen_user)
                .with_system(systems::visual::update_title_screen_camera)
                .with_system(resources::ui::ui_return_create_world)
                .with_system(resources::ui::ui_view_worlds)
        )
        .run();
}

fn log_setup() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        tracing_log::LogTracer::init().unwrap();
        let fmt_layer = if consts::DEV_BUILD {
            tracing_subscriber::fmt::Layer::default()
                .without_time()
                .with_file(false)
                .with_line_number(true)
        }
        else {
            tracing_subscriber::fmt::Layer::default()
                .without_time()
                .with_file(false)
                .with_line_number(false)
        };
        let subscriber = tracing_subscriber::Registry::default()
            .with(fmt_layer)
            .with(EnvFilter::new("INFO,wgpu=error,symphonia=error"));
        tracing::subscriber::set_global_default(subscriber)
            .expect("Couldn't set global tracing subscriber");
    }
    #[cfg(target_arch = "wasm32")]
    {
        tracing_wasm::set_as_global_default_with_config(
            tracing_wasm::WASMLayerConfigBuilder::new()
                .set_max_level(tracing::Level::WARN)
                .build()
        );
        console_error_panic_hook::set_once();
    }
}
