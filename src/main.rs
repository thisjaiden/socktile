use bevy::prelude::*;

use bevy_prototype_debug_lines::*;

mod components;
mod systems;
mod resources;
mod layers;
mod server;
mod client;
mod shared;

// Build switches
// --------------
// Is this an internal dev build?
pub const DEV_BUILD: bool      = true;
// Should UI debug lines be shown?
pub const DEBUG_UI: bool       = true;
// Should hitbox debug lines be shown?
pub const DEBUG_HITBOXES: bool = false;

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
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_system(systems::loading_screen.system())
        .add_system(systems::title_screen_spawner.system())
        .add_system(systems::title_screen_buttons.system())
        .add_system(systems::cursor.system())
        .add_system(systems::settings.system())
        .add_system(systems::join.system())
        .add_system(systems::join_ui_create.system())
        .add_system(systems::join_ui_update.system())
        .add_system(systems::create_user.system())
        .add_system(systems::create_user_ui.system())
        .add_system(systems::text_box.system())
        .add_system(systems::new.system())
        .add_system(systems::new_ui.system())
        .add_system(systems::new_exit.system())
        .add_system(systems::animator.system())
        .add_system(systems::netty_etick.system())
        .add_system(systems::netty_newtick.system())
        .add_system(systems::netty_reality.system())
        .insert_resource(resources::GameState::LoadingScreen)
        .insert_resource(resources::AssetHandles::init())
        .insert_resource(resources::TextBox::init())
        .insert_resource(resources::Animator::init())
        .insert_resource(systems::AnimatorTimer(Timer::from_seconds(1.0 / 60.0, true)))
        .insert_resource(resources::Netty::init())
        .insert_resource(resources::Reality::init())
        .run();
}