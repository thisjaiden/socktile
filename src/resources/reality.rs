use bevy::{prelude::*, utils::HashMap, render::camera::Camera as BevyCam};

use crate::{components::{GamePosition, ldtk::{PlayerMarker, TileMarker}}, shared::{terrain::TerrainState, netty::Packet, listing::GameListing}, ldtk::LDtkMap, MapAssets, GameState, FontAssets, layers::UI_TEXT};

use super::{Netty, ui::{UIManager, UIClickable, UIClickAction}};

pub struct Reality {
    player_position: GamePosition,
    camera: Camera,
    avalable_servers: Vec<GameListing>,
    push_servers: bool,
    chunks_to_load: Vec<(isize, isize)>,
    loaded_chunks: Vec<(isize, isize)>,
    buffered_chunks: HashMap<(isize, isize), Vec<(usize, usize, TerrainState)>>
}

impl Reality {
    pub fn init() -> Reality {
        Reality {
            player_position: GamePosition { x: 0.0, y: 0.0 },
            camera: Camera::PlayerPosition,
            avalable_servers: vec![],
            push_servers: false,
            chunks_to_load: vec![],
            loaded_chunks: vec![],
            buffered_chunks: HashMap::default()
        }
    }
    pub fn set_player_position(&mut self, position: GamePosition) {
        self.player_position = position;
        // load visible world
        const ENV_WIDTH: f64 = 1920.0;
        const ENV_HEIGHT: f64 = 1088.0;
        let tile_x = (self.player_position.x / ENV_WIDTH).round() as isize;
        let tile_y = (self.player_position.y / ENV_HEIGHT).round() as isize;
        self.chunks_to_load.push((tile_x, tile_y));
    }
    pub fn update_chunk(&mut self, chunk: (isize, isize)) {
        println!("Reality::update_chunk needs finishing");
    }
    pub fn add_chunk(&mut self, chunk_position: (isize, isize), chunk_data: Vec<(usize, usize, TerrainState)>) {
        println!("Reality::add_chunk needs finishing");
    }
    pub fn set_avalable_servers(&mut self, servers: Vec<GameListing>) {
        self.avalable_servers = servers;
        self.push_servers = true;
    }
    pub fn display_servers(&mut self) -> Option<Vec<GameListing>> {
        if self.push_servers {
            self.push_servers = false;
            return Some(self.avalable_servers.clone());
        }
        None
    }

    // Systems
    pub fn system_chunk_loader(
        mut selfs: ResMut<Reality>,
        mut commands: Commands,
        mut maps: ResMut<Assets<LDtkMap>>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        target_maps: Res<MapAssets>,
        fonts: Res<FontAssets>
    ) {
        if !selfs.chunks_to_load.is_empty() {
            for chunk in selfs.chunks_to_load.clone() {
                if !selfs.loaded_chunks.contains(&chunk) {
                    let a = maps.get_mut(target_maps.player.clone()).unwrap();
                    crate::ldtk::load_chunk(chunk, a, &mut texture_atlases, fonts.clone(), &mut commands);
                    selfs.loaded_chunks.push(chunk);
                }
            }
        }
        selfs.chunks_to_load.clear();
    }
    pub fn system_player_controls(
        mut selfs: ResMut<Reality>,
        mut netty: ResMut<Netty>,
        keyboard: Res<Input<KeyCode>>
    ) {
        let mut had_movement = false;
        let mut new_pos = selfs.player_position;
        // move
        if keyboard.pressed(KeyCode::W) {
            new_pos.y += 5.0;
            had_movement = true;
        }
        if keyboard.pressed(KeyCode::S) {
            new_pos.y -= 5.0;
            had_movement = true;
        }
        if keyboard.pressed(KeyCode::A) {
            new_pos.x -= 5.0;
            had_movement = true;
        }
        if keyboard.pressed(KeyCode::D) {
            new_pos.x += 5.0;
            had_movement = true;
        }
        // TODO: if collided, send back
        // send to server
        if had_movement {
            selfs.set_player_position(new_pos);
            netty.say(Packet::RequestMove(selfs.player_position));
        }
    }
    pub fn system_camera_updater(
        selfs: ResMut<Reality>,
        mut camera: Query<&mut Transform, With<BevyCam>>
    ) {
        let mut cam = camera.single_mut().unwrap();
        match &selfs.camera {
            Camera::PlayerPosition => {
                cam.translation.x = selfs.player_position.x as f32;
                cam.translation.y = selfs.player_position.y as f32;
            }
            Camera::Static(pos) => {
                cam.translation.x = pos.x as f32;
                cam.translation.y = pos.y as f32;
            }
            Camera::DriftToPlayer => {
                todo!();
            }
        }
    }
    pub fn system_player_locator(
        selfs: ResMut<Reality>,
        mut player: Query<&mut Transform, With<PlayerMarker>>
    ) {
        let mut location = player.single_mut().unwrap();
        location.translation.x = selfs.player_position.x as f32;
        location.translation.y = selfs.player_position.y as f32;
    }
    pub fn system_server_list_renderer(
        mut commands: Commands,
        mut selfs: ResMut<Reality>,
        mut uiman: ResMut<UIManager>,
        font_handles: Res<FontAssets>
    ) {
        if let Some(servers) = selfs.display_servers() {
            for (index, server) in servers.iter().enumerate() {
                commands.spawn_bundle(Text2dBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: server.public_name.clone(),
                                style: TextStyle {
                                    font: font_handles.simvoni.clone(),
                                    font_size: 35.0,
                                    color: Color::BLACK
                                }
                            }
                        ],
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center
                        }
                    },
                    transform: Transform::from_xyz(0.0, (1080.0 / 2.0) - 200.0 - (index as f32 * 128.0), UI_TEXT),
                    ..Default::default()
                }).insert(TileMarker {});
                uiman.add_ui(UIClickable {
                    action: UIClickAction::JoinWorld(server.internal_id),
                    location: (-200.0, ((1080.0 / 2.0) - 200.0 - (index as f32 * 128.0)) + 64.0),
                    size: (400.0, 128.0),
                    removed_on_use: false
                })
            }
        }
    }
}

pub enum Camera {
    PlayerPosition,
    Static(GamePosition),
    DriftToPlayer,
}