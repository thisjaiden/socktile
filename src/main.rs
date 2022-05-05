#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod components;
mod systems;
mod resources;
mod consts;
mod server;
mod shared;
mod window_setup;
mod ldtk;
mod assets;


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
    if consts::DEV_BUILD {
        // Allow GGS to be run if it's a dev build, and warn about distribution.
        println!("\x1B[40;91mWARNING: This is an internal build. All software is property of and (c) Jaiden Bernard. Do not share this software without permission from the property owners.\x1B[0m");
        println!("Sidenote: if you just built this from GitHub, do as you will. This doesn't apply to you.");
    }
    if consts::ALLOW_GGS {
        let mut args = std::env::args();
        args.next();
        if let Some(argument) = args.next() {
            if argument == "--ggs" {
                println!("\x1B[40;91mWARNING: Running as a GGS. Make sure you know what you're doing!\x1B[0m");
                server::startup();
            }
        }
    }

    // Create our Bevy app!
    let mut app = App::new();
    // Use embedded assets
    app.add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
    });
    // Register all the assets we need loaded.
    AssetLoader::new(GameState::Load)
        .continue_to_state(GameState::NetworkCheck)
        .with_collection::<assets::MapAssets>()
        .with_collection::<assets::FontAssets>()
        .with_collection::<assets::AnimatorAssets>()
        .with_collection::<assets::UIAssets>()
        .with_collection::<assets::ItemAssets>()
        .with_collection::<assets::ObjectAssets>()
        .build(&mut app);
    // Add plugins and systems to our app, then run it!
    app
        //.add_plugin(bevy_kira_audio::AudioPlugin)
        .add_plugin(ldtk::LDtkPlugin)
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
                .with_system(resources::Netty::system_server_list)
        )
        .add_system_set(
            SystemSet::on_update(GameState::ServerList)
                .with_system(resources::Reality::system_server_list_renderer)
        )
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
                .with_system(resources::Animator::system_player_animator)
                .with_system(resources::Chat::system_display_chat)
                .with_system(resources::Chat::system_pull_messages)
                .with_system(resources::Chat::system_open_chat)
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
