use bevy::prelude::*;

mod components;
mod systems;
mod resources;
mod layers;
mod server;
mod client;
mod shared;

const DEV_BUILD: bool = true;
const GGS_BUILD: bool = false;

fn main() {
    if DEV_BUILD {
        println!("\x1B[40;91mTHIS IS AN INTERNAL BUILD. DO NOT DISTRIBUTE.\x1B[0m");
    }
    if GGS_BUILD {
        println!("\x1B[40;91mTHIS IS A GLOBAL GAME SERVER BUILD. DO NOT DISTRIBUTE. DO NOT RUN FROM THE WRONG LOCATION.\x1B[0m");
        server::core::startup();
    }
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(systems::loading_screen.system())
        .add_system(systems::title_screen.system())
        .add_system(systems::title_screen_buttons.system())
        .add_system(systems::cursor.system())
        .add_system(systems::settings.system())
        .add_system(systems::join.system())
        .add_system(systems::join_ui_create.system())
        .add_system(systems::join_ui_update.system())
        .add_system(systems::join_network.system())
        .add_system(systems::create_user.system())
        .add_system(systems::create_user_ui.system())
        .add_system(systems::text_box.system())
        .add_system(systems::new.system())
        .insert_resource(resources::GameState::LoadingScreen)
        .insert_resource(resources::AssetHandles::init())
        .insert_resource(resources::TextBox::init())
        .run();
}

