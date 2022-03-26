use bevy::{prelude::*, render::camera::Camera, utils::HashMap};

use crate::{components::{GamePosition, ldtk::{PlayerMarker, TileMarker, Tile}, PauseMenuMarker, UILocked}, shared::{terrain::TerrainState, netty::Packet, listing::GameListing, saves::User, player::{PlayerData, Inventory}}, ldtk::LDtkMap, assets::{MapAssets, FontAssets, AnimatorAssets}, consts::{UI_TEXT, PLAYER_CHARACTERS}};

use super::{Netty, ui::{UIManager, UIClickable, UIClickAction}, Disk, chat::ChatMessage};

pub struct Reality {
    /// Player's current position
    player_position: GamePosition,
    /// Player's data
    player: PlayerData,
    /// Queued chat messages
    chat_messages: Vec<ChatMessage>,
    /// Servers that can be joined
    avalable_servers: Vec<GameListing>,
    push_servers: bool,
    /// Chunks that need to be loaded
    chunks_to_load: Vec<(isize, isize)>,
    /// Players to spawn in and load
    players_to_spawn: Vec<(User, GamePosition)>,
    /// Players to unload
    players_to_despawn: Vec<User>,
    /// Chunks that are currently loaded
    loaded_chunks: Vec<(isize, isize)>,
    /// Is this server owned by the active player?
    owns_server: bool,
    pause_menu: MenuState,
    players_to_move: HashMap<User, GamePosition>
}

impl Reality {
    pub fn init() -> Reality {
        Reality {
            player_position: GamePosition::zero(),
            player: PlayerData::new(),
            chat_messages: vec![],
            avalable_servers: vec![],
            push_servers: false,
            chunks_to_load: vec![],
            players_to_spawn: vec![],
            players_to_despawn: vec![],
            loaded_chunks: vec![],
            owns_server: false,
            pause_menu: MenuState::Closed,
            players_to_move: HashMap::default()
        }
    }
    pub fn reset(&mut self) {
        *self = Reality::init();
    }
    pub fn pause_closed(&mut self) {
        self.pause_menu = MenuState::Closed;
    }
    pub fn queue_player_move(&mut self, p: User, l: GamePosition) {
        self.players_to_move.insert(p, l);
    }
    pub fn set_inventory(&mut self, inventory: Inventory) {
        self.player.inventory = inventory;
    }
    pub fn pull_messages(&mut self) -> Vec<ChatMessage> {
        let a = self.chat_messages.clone();
        self.chat_messages.clear();
        return a;
    }
    pub fn queue_chat(&mut self, msg: ChatMessage) {
        self.chat_messages.push(msg);
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
        for chunk in selfs.chunks_to_load.clone() {
            if !selfs.loaded_chunks.contains(&chunk) {
                println!("Loading chunk at ({:?})", chunk);
                let a = maps.get_mut(target_maps.core.clone()).unwrap();
                crate::ldtk::load_chunk(chunk, a, &mut texture_atlases, fonts.clone(), &mut commands);
                selfs.loaded_chunks.push(chunk);
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
    pub fn system_pause_menu(
        mut selfs: ResMut<Reality>,
        mut uiman: ResMut<UIManager>,
        keyboard: Res<Input<KeyCode>>
    ) {
        if keyboard.just_pressed(KeyCode::Escape) {
            if selfs.pause_menu == MenuState::Closed {
                selfs.pause_menu = MenuState::Queued;
            }
            else {
                selfs.pause_menu = MenuState::Closed;
                uiman.reset_ui();
            }
        }
    }
    pub fn system_player_controls(
        mut selfs: ResMut<Reality>,
        mut netty: ResMut<Netty>,
        keyboard: Res<Input<KeyCode>>,
        tiles: Query<&mut Tile>
    ) {
        if keyboard.any_pressed([KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D]) {
            let centered_chunk = (
                ((selfs.player_position.x + (1920.0 / 2.0)) / 1920.0) as isize,
                ((selfs.player_position.y + (1088.0 / 2.0)) / 1088.0) as isize
            );
            let centered_tile = (
                ((selfs.player_position.x - (1920 * centered_chunk.0) as f64 + (1920.0 / 2.0)) / 64.0) as isize,
                ((selfs.player_position.y - (1088 * centered_chunk.1) as f64 + (1088.0 / 2.0)) / 64.0) as isize + 1
            );
            let needed_tiles = [
                centered_tile,
                (centered_tile.0, centered_tile.1 + 1),
                (centered_tile.0, centered_tile.1 - 1),
                (centered_tile.0 + 1, centered_tile.1 + 1),
                (centered_tile.0 + 1, centered_tile.1 - 1),
                (centered_tile.0 + 1, centered_tile.1),
                (centered_tile.0 - 1, centered_tile.1 + 1),
                (centered_tile.0 - 1, centered_tile.1 - 1),
                (centered_tile.0 - 1, centered_tile.1),
            ];
            let mut needed_pairs = vec![];
            let mut needed_chunks = vec![];
            for tile in needed_tiles {
                if tile.0 == -1 && tile.1 != 0 {
                    needed_pairs.push(((centered_chunk.0 - 1, centered_chunk.1), (29, tile.1 as usize)));
                    if !needed_chunks.contains(&(centered_chunk.0 - 1, centered_chunk.1)) {
                        needed_chunks.push((centered_chunk.0 - 1, centered_chunk.1));
                    }
                }
                else if tile.0 != -1 && tile.1 == 0 {
                    needed_pairs.push(((centered_chunk.0, centered_chunk.1 - 1), (tile.0 as usize, 17)));
                    if !needed_chunks.contains(&(centered_chunk.0, centered_chunk.1 - 1)) {
                        needed_chunks.push((centered_chunk.0, centered_chunk.1 - 1));
                    }
                }
                else if tile.0 == -1 && tile.1 == 0 {
                    needed_pairs.push(((centered_chunk.0 - 1, centered_chunk.1 - 1), (29, 17)));
                    if !needed_chunks.contains(&(centered_chunk.0 - 1, centered_chunk.1 - 1)) {
                        needed_chunks.push((centered_chunk.0 - 1, centered_chunk.1 - 1));
                    }
                }
                else {
                    needed_pairs.push((centered_chunk, (tile.0 as usize, tile.1 as usize)));
                    if !needed_chunks.contains(&centered_chunk) {
                        needed_chunks.push(centered_chunk);
                    }
                }
            }
            let mut pulled_tiles = vec![];
            tiles.for_each(|tile| {
                if needed_chunks.contains(&tile.chunk) {
                    for (chunk, n_tile) in &needed_pairs {
                        if tile.chunk == *chunk && tile.position == *n_tile {
                            pulled_tiles.push(tile);
                        }
                    }
                }
            });
            let mut had_movement = false;
            let mut new_pos = selfs.player_position;
            // move
            if keyboard.pressed(KeyCode::W) {
                new_pos.y += 4.0;
                if !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.x, new_pos.y)) {
                    had_movement = true;
                }
                else {
                    new_pos.y -= 4.0;
                }
            }
            if keyboard.pressed(KeyCode::S) {
                new_pos.y -= 4.0;
                if !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.x, new_pos.y)) {
                    had_movement = true;
                }
                else {
                    new_pos.y += 4.0;
                }
            }
            if keyboard.pressed(KeyCode::A) {
                new_pos.x -= 4.0;
                if !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.x, new_pos.y)) {
                    had_movement = true;
                }
                else {
                    new_pos.x += 4.0;
                }
            }
            if keyboard.pressed(KeyCode::D) {
                new_pos.x += 4.0;
                if !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.x, new_pos.y)) {
                    had_movement = true;
                }
                else {
                    new_pos.x -= 4.0;
                }
            }
            
            // send to server
            if had_movement {
                selfs.set_player_position(new_pos);
                netty.say(Packet::RequestMove(selfs.player_position));
            }
        }
    }
    pub fn system_pause_renderer(
        mut commands: Commands,
        mut selfs: ResMut<Reality>,
        mut uiman: ResMut<UIManager>,
        fonts: Res<FontAssets>,
        mut desps: Query<(Entity, &mut Transform, &PauseMenuMarker)>
    ) {
        match selfs.pause_menu {
            MenuState::Closed => {
                // Despawn any alive menu objects/ui
                desps.for_each(|(despawn, _, _)| {
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
                }).insert(PauseMenuMarker { type_: 1 }).insert(UILocked {});
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
                desps.for_each_mut(|(_, mut loc, type_)| {
                    match type_.type_ {
                        1 => {
                            loc.translation.x = 0.0;
                            loc.translation.y = 0.0;
                        }
                        2 => {
                            loc.translation.x = 0.0;
                            loc.translation.y = 100.0;
                        }
                        t => {
                            println!("WARNING: Pause menu component has unkown type {t}!");
                        }
                    }
                });
            }
        }
    }
    pub fn system_pause_invite(
        mut tb: ResMut<crate::resources::TextBox>,
        mut netty: ResMut<Netty>,
        mut selfs: ResMut<Reality>,
        mut tbe: Query<&mut Text, With<crate::components::TextBox>>
    ) {
        tbe.for_each_mut(|mut textable| {
            textable.sections[0].value = tb.grab_buffer();
            if !tb.grab_buffer().contains('#') {
                textable.sections[1].value = String::from("#????");
            }
            else {
                textable.sections[1].value = String::new();
            }
        });
        if tb.grab_buffer().contains('\n') {
            if !tb.grab_buffer().contains('#') {
                // do nothing, invalid without a tag
                tb.eat_buffer();
                return
            }
            let mut strs = tb.grab_buffer();
            strs = String::from(strs.trim_end_matches('\n'));
            let tag = strs.split('#').nth(1).unwrap().parse::<u16>();
            if let Ok(val) = tag {
                netty.say(Packet::WhitelistUser(User {
                    username: tb.grab_buffer().split('#').nth(0).unwrap().to_string(),
                    tag: val
                }));
            }
            else {
                selfs.queue_chat(ChatMessage { text: String::from("Invalid user tag."), color: Color::RED, sent_at: std::time::Instant::now() });
            }
            tb.clear_buffer();
            selfs.pause_closed();
        }
    }
    pub fn system_camera_updater(
        selfs: Res<Reality>,
        mut queries: QuerySet<(
            QueryState<&mut Transform, With<Camera>>,
            QueryState<&mut Transform, With<UILocked>>
        )>
    ) {
        queries.q0().for_each_mut(|mut campos| {
            campos.translation.x = selfs.player_position.x as f32;
            campos.translation.y = selfs.player_position.y as f32;
            if selfs.loaded_chunks.is_empty() {
                campos.translation.x = 0.0;
                campos.translation.y = 0.0;
            }
        });
        queries.q1().for_each_mut(|mut transform| {
            transform.translation.x += selfs.player_position.x as f32;
            transform.translation.y += selfs.player_position.y as f32;
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

/// true if collided, false otherwise
fn calc_player_against_tiles(tiles: &[&Tile], player: (f64, f64)) -> bool {
    for tile in tiles {
        let offset_x = (-1920.0 / 2.0) + (tile.chunk.0 as f64 * 1920.0) + ((tile.position.0 as f64) * 64.0);
        let offset_y = (-1088.0 / 2.0) + (tile.chunk.1 as f64 * 1088.0) + ((tile.position.1 as f64 - 1.0) * 64.0);
        let mut state = TerrainState {
            tileset: tile.sprite.0,
            tile: tile.sprite.1
        };
        if state.collides(player, offset_x, offset_y) {
            return true;
        }
    }
    false
}
