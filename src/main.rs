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
        println!("THIS IS AN INTERNAL BUILD. DO NOT DISTRIBUTE.");
    }
    if GGS_BUILD {
        println!("THIS IS A GLOBAL GAME SERVER BUILD. DO NOT DISTRIBUTE. DO NOT RUN FROM THE WRONG LOCATION.");
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
        .add_system(systems::join_ui.system())
        .add_system(systems::join_network.system())
        .add_system(systems::create_user.system())
        .insert_resource(resources::GameState::LoadingScreen)
        .insert_resource(resources::AssetHandles::init())
        .run();
}

