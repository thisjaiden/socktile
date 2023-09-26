#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use crate::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use iyes_progress::ProgressPlugin;
use tracing_subscriber::{prelude::*, EnvFilter};

mod assets;
mod components;
mod consts;
mod matrix;
mod modular_assets;
mod prelude;
mod resources;
mod server;
mod shared;
mod systems;
mod utils;
mod window_setup;

/// Represents the state the game is currently in. Used to decide which systems
/// to run.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
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

impl Default for GameState {
    fn default() -> Self {
        Self::PreLoadLoad
    }
}

fn main() {
    // Set up the logger. We do this without bevy because the server doesn't set
    // up a bevy app.
    log_setup();

    // Warn about distribution of internal builds
    if consts::DEV_BUILD {
        info!("This is an internal build. All software is property of and (c) Jaiden Bernard 2021-2023.");
        info!("Do not share this software without permission from the property owners.");
    }
    // The following section pertains to game servers, which cannot be run on WASM.
    #[cfg(not(target_arch = "wasm32"))]
    {
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
                // Run a server.This returns a never type and should never 
                // proceed to launching a normal game
                info!("Running as a server. Make sure you know what you're doing!");
                server::startup(arguments);
            }
        }
    }

    // Create our Bevy app!
    let mut app = App::new();

    // Enable embedded assets through `bevy_embedded_assets`
    if EMBED_ASSETS {
        app.add_plugins(
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
                .disable::<bevy::log::LogPlugin>(),
        );
    } else {
        app.add_plugins(DefaultPlugins.build().disable::<bevy::log::LogPlugin>());
    }

    // Add assets, plugins and systems to our app, then run it
    app
        // The initial state is pulled from `GameState::default()`
        .add_state::<GameState>()
        // Add a loading state for assets
        .add_loading_state(LoadingState::new(GameState::Load))
        .add_collection_to_loading_state::<_, assets::CoreAssets>(GameState::Load)
        .add_collection_to_loading_state::<_, assets::FontAssets>(GameState::Load)
        .add_collection_to_loading_state::<_, assets::AnimatorAssets>(GameState::Load)
        .add_collection_to_loading_state::<_, assets::UIAssets>(GameState::Load)
        .add_collection_to_loading_state::<_, assets::ItemAssets>(GameState::Load)
        .add_collection_to_loading_state::<_, assets::ObjectAssets>(GameState::Load)
        .add_collection_to_loading_state::<_, assets::NPCAssets>(GameState::Load)
        .add_plugins((
            ProgressPlugin::new(GameState::Load).continue_to(GameState::NetworkCheck),
            modular_assets::ModularAssetsPlugin,
            bevy_kira_audio::AudioPlugin,
            bevy_easings::EasingsPlugin,
            bevy_prototype_debug_lines::DebugLinesPlugin::default(),
        ))
        // Unrendered background is white
        .insert_resource(ClearColor(Color::WHITE))
        // Spawn logo and loading bar
        .add_systems(Startup, systems::visual::logo)
        // Load the window settings and audio devices when loading assets
        .add_systems(OnEnter(GameState::Load), (
            window_setup::window_setup,
            systems::audio::audio_setup,
        ))
        // Display the progress of loading assets
        .add_systems(Update, systems::visual::loading_prog
            .run_if(in_state(GameState::Load)))
        // Spawn the cursor and attempt to connect to the GGS
        .add_systems(OnEnter(GameState::NetworkCheck), (
            resources::network::system_startup_checks,
            systems::cursor::spawn,
        ))
        // [ORDERED] Spawn the titlescreen textures/text and clear any old stuff
        .add_systems(OnEnter(GameState::TitleScreen), (
            systems::visual::clear_old,
            systems::visual::title_screen,
            systems::audio::title_screen_loop,
        ).chain())
        // [ORDERED] Spawn the user creation textures/text and clear any old stuff
        .add_systems(OnEnter(GameState::MakeUser), (
            systems::visual::clear_old,
            systems::visual::make_user,
        ).chain())
        // Update user creation text and UI
        .add_systems(Update, (
            systems::text_box::user_creation
        ).run_if(in_state(GameState::MakeUser)))
        // Open the settings menu
        .add_systems(OnEnter(GameState::Settings), (
            systems::visual::settings_video,
            resources::ui::ui_settings_camera,
        ))
        // Update the settings menu UI
        .add_systems(Update, (
            resources::ui::ui_settings_tab,
            resources::ui::ui_toggle_fullscreen,
            resources::ui::ui_increase_scaling,
            resources::ui::ui_decrease_scaling,
            resources::ui::ui_return_titlescreen,
            resources::ui::ui_settings_text_updater,
        ).run_if(in_state(GameState::Settings)))
        .add_systems(OnEnter(GameState::MakeGame), (
            systems::visual::clear_old,
            systems::visual::create_world,
            systems::text_box::game_creation_once,
        ).chain())
        .add_systems(Update, (
            systems::text_box::game_creation,
            resources::ui::ui_return_titlescreen,
        ).run_if(in_state(GameState::MakeGame)))
        .add_systems(OnEnter(GameState::ServerList), (
            systems::visual::clear_old,
            systems::visual::join_world,
            resources::network::system_server_list,
        ).chain())
        .add_systems(Update, (
            resources::Reality::system_server_list_renderer,
            resources::ui::ui_game,
            resources::ui::ui_return_titlescreen,
        ).run_if(in_state(GameState::ServerList)))
        .add_systems(OnEnter(GameState::Play), (
            systems::visual::clear_old,
            resources::Reality::system_spawn_hotbar,
            resources::Chat::system_init,
        ).chain())
        .add_systems(Update, (
            systems::visual::update_title_screen_user,
            systems::visual::update_title_screen_camera,
            resources::ui::ui_return_create_world,
            resources::ui::ui_view_worlds,
        ).run_if(in_state(GameState::TitleScreen)))
        .add_systems(Update, (
            resources::ui::ui_resume_game_settings,
            systems::visual::clear_settings
            ).run_if(state_changed::<GameState>())
            .run_if(in_state(GameState::Play)))
        .add_systems(Update, (
            systems::cursor::cursor,
            resources::ui::ui_manager,
        ).chain())
        .add_systems(Update, (
            window_setup::window_update,
            systems::text_box::text_input,
            resources::network::system_step,
            resources::ui::ui_open_settings,
            resources::ui::ui_quick_exit,
            resources::ui::ui_close_pause_menu,
            resources::ui::ui_invite_menu,
            resources::ui::ui_close_settings,
            resources::ui::ui_debug_lines,
        ))
        .add_systems(Update, (
            resources::last_state::system_update_last_state
                .run_if(state_changed::<GameState>()),
            resources::last_state::system_update_last_state_live
        ).chain())
        .insert_resource(resources::Reality::init())
        .insert_resource(resources::Animator::init())
        .insert_resource(resources::TextBox::init())
        .insert_resource(resources::ui::UIManager::init())
        .insert_resource(resources::Disk::init())
        .insert_resource(resources::Chat::init())
        .insert_resource(resources::LastState::init())
        .add_systems(Update, (
            resources::Reality::system_spawn_objects,
            resources::Reality::system_npc_interaction,
            resources::Reality::system_player_loader,
            resources::Reality::system_player_unloader,
            resources::Reality::system_scroll_hotbar,
            resources::Reality::system_pause_invite,
            resources::Reality::system_update_objects,
            resources::Reality::system_remove_objects,
            resources::Reality::system_update_hotbar,
            resources::Reality::system_hitbox_debug_lines,
            resources::Reality::system_player_debug_lines,
            resources::Reality::system_action_none,
            resources::Reality::system_action_chop,
            resources::Reality::system_start_npc_popups,
            resources::Reality::system_shrink_npc_popups,
            resources::Animator::system_player_animator,
            resources::Animator::system_player_initiator,
            resources::Chat::system_display_chat,
            resources::Chat::system_pull_messages,
            resources::Chat::system_open_chat,
            
        ).run_if(in_state(GameState::Play)))
        // TODO: FIXME: numerical limit of 20 systems in one .add_systems call
        // keep an eye on if bevy increases/fixes this. Used to be 15!
        .add_systems(Update, (
            resources::Chat::system_type_chat,
            resources::Chat::system_send_chat,
            resources::ui::ui_forward,
            resources::ui::ui_disconnect_game,
            systems::visual::animate_sprites,
        ).run_if(in_state(GameState::Play)))
        .add_systems(Update, (
            resources::Reality::system_action_blueprint,
            resources::Reality::system_chunk_derenderer,
            resources::Reality::system_mark_chunks,
            resources::Reality::system_render_waiting_chunks,
            // ...
            resources::Reality::system_pause_menu,
            resources::Reality::system_center_dialouge_text,
            resources::Reality::system_player_controls,
            resources::Reality::system_pause_renderer,
            resources::Reality::system_position_hotbar,
            resources::Reality::system_player_locator,
            resources::Reality::system_display_blueprint,
            resources::Reality::system_camera_updater,
        ).chain().run_if(in_state(GameState::Play)))
        .run();
}

/// Sets up logging using `tracing` to support both desktop and WASM platforms.
fn log_setup() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        tracing_log::LogTracer::init().unwrap();
        let fmt_layer = if consts::DEV_BUILD {
            tracing_subscriber::fmt::Layer::default()
                .without_time()
                .with_file(false)
                .with_line_number(true)
        } else {
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
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default_with_config(
            tracing_wasm::WASMLayerConfigBuilder::new()
                .set_max_level(tracing::Level::WARN)
                .build(),
        );
    }
}
