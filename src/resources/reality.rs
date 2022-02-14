use bevy::{prelude::*, render::camera::Camera, utils::HashMap};

use crate::{components::{GamePosition, ldtk::{PlayerMarker, TileMarker}, PauseMenuMarker}, shared::{terrain::TerrainState, netty::Packet, listing::GameListing, saves::User}, ldtk::{LDtkMap, CollisionMapPart, CollisionMap, CollisionState}, assets::{MapAssets, FontAssets, AnimatorAssets}, consts::{UI_TEXT, PLAYER_CHARACTERS}};

use super::{Netty, ui::{UIManager, UIClickable, UIClickAction}, Disk};

pub struct Reality {
    player_position: GamePosition,
    avalable_servers: Vec<GameListing>,
    push_servers: bool,
    chunks_to_load: Vec<(isize, isize)>,
    players_to_spawn: Vec<(User, GamePosition)>,
    players_to_despawn: Vec<User>,
    loaded_chunks: Vec<(isize, isize)>,
    owns_server: bool,
    pause_menu: MenuState,
    collision_map: CollisionMap,
    players_to_move: HashMap<User, GamePosition>
}

impl Reality {
    pub fn init() -> Reality {
        Reality {
            player_position: GamePosition { x: 0.0, y: 0.0 },
            avalable_servers: vec![],
            push_servers: false,
            chunks_to_load: vec![],
            players_to_spawn: vec![],
            players_to_despawn: vec![],
            loaded_chunks: vec![],
            owns_server: false,
            pause_menu: MenuState::Closed,
            collision_map: CollisionMap::new(),
            players_to_move: HashMap::default()
        }
    }
    pub fn reset(&mut self) {
        *self = Reality::init();
    }
    pub fn no_collision(&mut self) -> bool {
        !self.collision_map.has_stuff()
    }
    pub fn cmappt_new(&mut self, cmappt: CollisionMapPart) {
        self.collision_map.add_part(cmappt);
    }
    pub fn get_point(&mut self, pt: GamePosition) -> CollisionState {
        self.collision_map.point_is(pt)
    }
    pub fn pause_closed(&mut self) {
        self.pause_menu = MenuState::Closed;
    }
    pub fn queue_player_move(&mut self, p: User, l: GamePosition) {
        self.players_to_move.insert(p, l);
    }
    pub fn set_player_position(&mut self, position: GamePosition) {
        self.player_position = position;
        // load visible world
        const ENV_WIDTH: f64 = 1920.0;
        const ENV_HEIGHT: f64 = 1088.0;
        let tile_x = (self.player_position.x / ENV_WIDTH).round() as isize;
        let tile_y = (self.player_position.y / ENV_HEIGHT).round() as isize;
        self.chunks_to_load.push((tile_x, tile_y));
        self.chunks_to_load.push((tile_x, tile_y + 1));
        self.chunks_to_load.push((tile_x, tile_y - 1));
        self.chunks_to_load.push((tile_x + 1, tile_y));
        self.chunks_to_load.push((tile_x - 1, tile_y));
    }
    pub fn set_ownership(&mut self, ownership: bool) {
        self.owns_server = ownership;
    }
    pub fn update_chunk(&mut self, _chunk: (isize, isize)) {
        println!("Reality::update_chunk needs finishing");
    }
    pub fn add_chunk(&mut self, _chunk_position: (isize, isize), _chunk_data: Vec<(usize, usize, TerrainState)>) {
        println!("Reality::add_chunk needs finishing");
    }
    pub fn add_online_players(&mut self, players: Vec<(User, GamePosition)>) {
        for (euser, pos) in players {
            self.players_to_spawn.push((euser, pos));
        }
    }
    pub fn disconnect_player(&mut self, player: User) {
        self.players_to_despawn.push(player);
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
                    println!("Loading chunk at ({:?})", chunk);
                    let a = maps.get_mut(target_maps.core.clone()).unwrap();
                    let cmappt = crate::ldtk::load_chunk(chunk, a, &mut texture_atlases, fonts.clone(), &mut commands);
                    selfs.cmappt_new(cmappt);
                    selfs.loaded_chunks.push(chunk);
                }
            }
        }
        selfs.chunks_to_load.clear();
    }
    pub fn system_player_loader(
        mut selfs: ResMut<Reality>,
        assets: Res<AnimatorAssets>,
        disk: Res<Disk>,
        mut commands: Commands
    ) {
        for (user, location) in selfs.players_to_spawn.clone() {
            if user != disk.user().unwrap() {
                commands.spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(location.x as f32, location.y as f32, PLAYER_CHARACTERS),
                    texture: assets.placeholder.clone(),
                    ..Default::default()
                }).insert(PlayerMarker { user, isme: false });
            }
        }
        selfs.players_to_spawn.clear();
    }
    pub fn system_player_unloader(
        mut selfs: ResMut<Reality>,
        mut unloads: Query<(Entity, &mut PlayerMarker)>,
        mut commands: Commands
    ) {
        unloads.for_each_mut(|(e, m)| {
            if selfs.players_to_despawn.contains(&m.user) {
                commands.entity(e).despawn();
            }
        });
        selfs.players_to_despawn.clear();
    }
    pub fn system_player_controls(
        mut selfs: ResMut<Reality>,
        mut netty: ResMut<Netty>,
        mut uiman: ResMut<UIManager>,
        keyboard: Res<Input<KeyCode>>
    ) {
        if selfs.no_collision() {
            return;
        }
        let mut had_movement = false;
        let mut new_pos = selfs.player_position;
        // move
        if keyboard.pressed(KeyCode::W) {
            new_pos.y += 4.0;
            had_movement = true;
        }
        if keyboard.pressed(KeyCode::S) {
            new_pos.y -= 4.0;
            had_movement = true;
        }
        if keyboard.pressed(KeyCode::A) {
            new_pos.x -= 4.0;
            had_movement = true;
        }
        if keyboard.pressed(KeyCode::D) {
            new_pos.x += 4.0;
            had_movement = true;
        }
        if keyboard.just_pressed(KeyCode::Escape) {
            if selfs.pause_menu == MenuState::Closed {
                selfs.pause_menu = MenuState::Queued;
            }
            else {
                selfs.pause_menu = MenuState::Closed;
                uiman.reset_ui();
            }
        }
        // TODO: if collided, send back
        let o_l_tl = GamePosition {
            x: selfs.player_position.x - 30.0,
            y: selfs.player_position.y + 30.0
        };
        let o_l_tr = GamePosition {
            x: selfs.player_position.x + 30.0,
            y: selfs.player_position.y + 30.0
        };
        let o_l_bl = GamePosition {
            x: selfs.player_position.x - 30.0,
            y: selfs.player_position.y - 30.0
        };
        let o_l_br = GamePosition {
            x: selfs.player_position.x + 30.0,
            y: selfs.player_position.y - 30.0
        };
        let o_p_tl = selfs.get_point(o_l_tl);
        let o_p_tr = selfs.get_point(o_l_tr);
        let o_p_bl = selfs.get_point(o_l_bl);
        let o_p_br = selfs.get_point(o_l_br);
        let o_arr = [o_p_tl, o_p_tr, o_p_bl, o_p_br];
        let n_l_tl = GamePosition {
            x: new_pos.x - 30.0,
            y: new_pos.y + 30.0
        };
        let n_l_tr = GamePosition {
            x: new_pos.x + 30.0,
            y: new_pos.y + 30.0
        };
        let n_l_bl = GamePosition {
            x: new_pos.x - 30.0,
            y: new_pos.y - 30.0
        };
        let n_l_br = GamePosition {
            x: new_pos.x + 30.0,
            y: new_pos.y - 30.0
        };
        let n_p_tl = selfs.get_point(n_l_tl);
        let n_p_tr = selfs.get_point(n_l_tr);
        let n_p_bl = selfs.get_point(n_l_bl);
        let n_p_br = selfs.get_point(n_l_br);
        let n_arr = [n_p_tl, n_p_tr, n_p_bl, n_p_br];
        if n_arr.contains(&CollisionState::Wall) {
            had_movement = false;
        }
        if n_arr.contains(&CollisionState::Elevated) && o_arr.contains(&CollisionState::Ground) {
            had_movement = false
        }
        if n_arr.contains(&CollisionState::Ground) && o_arr.contains(&CollisionState::Elevated) {
            had_movement = false;
        }
        if !o_arr.contains(&CollisionState::Water) && n_arr.contains(&CollisionState::Water) {
            had_movement = false
        }
        // send to server
        if had_movement {
            selfs.set_player_position(new_pos);
            netty.say(Packet::RequestMove(selfs.player_position));
        }
    }
    pub fn system_pause_renderer(
        mut commands: Commands,
        mut selfs: ResMut<Reality>,
        mut uiman: ResMut<UIManager>,
        fonts: Res<FontAssets>,
        desps: Query<Entity, With<PauseMenuMarker>>
    ) {
        match selfs.pause_menu {
            MenuState::Closed => {
                // Despawn any alive menu objects/ui
                desps.for_each(|despawn| {
                    commands.entity(despawn).despawn();
                });
            }
            MenuState::Queued => {
                // Spawn menu
                let m_color = if selfs.owns_server {
                    Color::BLACK
                }
                else {
                    Color::GRAY
                };
                commands.spawn_bundle(Text2dBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: String::from("Resume\n"),
                                style: TextStyle {
                                    font: fonts.simvoni.clone(),
                                    font_size: 55.0,
                                    color: Color::BLACK
                                }
                            },
                            TextSection {
                                value: String::from("Invite\n"),
                                style: TextStyle {
                                    font: fonts.simvoni.clone(),
                                    font_size: 55.0,
                                    color: m_color
                                }
                            },
                            TextSection {
                                value: String::from("Settings\nExit"),
                                style: TextStyle {
                                    font: fonts.simvoni.clone(),
                                    font_size: 55.0,
                                    color: Color::BLACK
                                }
                            }
                        ],
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center
                        }
                    },
                    transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
                    ..Default::default()
                }).insert(PauseMenuMarker {});
                uiman.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("ClosePauseMenu")),
                    location: (-150.0, 110.0 - 27.5),
                    size: (300.0, 55.0),
                    removed_on_use: false
                });
                uiman.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("InvitePlayer")),
                    location: (-150.0, 55.0 - 27.5),
                    size: (300.0, 55.0),
                    removed_on_use: false
                });
                uiman.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("OpenSettings")),
                    location: (-150.0, -27.5),
                    size: (300.0, 55.0),
                    removed_on_use: false
                });
                uiman.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("LeaveGame")),
                    location: (-150.0, -55.0 - 27.5),
                    size: (300.0, 55.0),
                    removed_on_use: false
                });
                selfs.pause_menu = MenuState::Open;
            }
            MenuState::Open => {
                // Update menu (if applicable)
            }
        }
    }
    pub fn system_pause_invite(
        mut tb: ResMut<crate::resources::TextBox>,
        mut netty: ResMut<Netty>,
        mut selfs: ResMut<Reality>
    ) {
        if tb.grab_buffer().contains('\n') {
            if !tb.grab_buffer().contains('#') {
                // do nothing, invalid
                // TODO: tell the user about it
                return
            }
            let mut strs = tb.grab_buffer();
            strs = String::from(strs.trim_end_matches('\n'));
            netty.say(Packet::WhitelistUser(User {
                username: tb.grab_buffer().split('#').nth(0).unwrap().to_string(),
                tag: strs.split('#').nth(1).unwrap().parse::<u16>().unwrap()
            }));
            tb.clear_buffer();
            selfs.pause_closed();
        }
    }
    pub fn system_camera_updater(
        selfs: ResMut<Reality>,
        mut camera: Query<&mut Transform, Or<(With<Camera>, With<PauseMenuMarker>)>>
    ) {
        camera.for_each_mut(|mut campos| {
            campos.translation.x = selfs.player_position.x as f32;
            campos.translation.y = selfs.player_position.y as f32;
            if selfs.loaded_chunks.is_empty() {
                campos.translation.x = 0.0;
                campos.translation.y = 0.0;
            }
        });
    }
    pub fn system_player_locator(
        mut selfs: ResMut<Reality>,
        mut player: Query<(&mut Transform, &mut PlayerMarker)>
    ) {
        player.for_each_mut(|(mut l, m)| {
            if m.isme {
                l.translation.x = selfs.player_position.x as f32;
                l.translation.y = selfs.player_position.y as f32;
            }
            if selfs.players_to_move.contains_key(&m.user) {
                let which = selfs.players_to_move.get(&m.user).unwrap();
                l.translation.x = which.x as f32;
                l.translation.y = which.y as f32;
            }
        });
        selfs.players_to_move.clear();
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuState {
    Closed,
    Queued,
    Open
}
