use bevy::{prelude::*, render::camera::Camera, utils::HashMap, input::mouse::{MouseWheel, MouseScrollUnit}};
use uuid::Uuid;

use crate::{components::{GamePosition, ldtk::{PlayerMarker, TileMarker, Tile}, PauseMenuMarker, UILocked, HotbarMarker}, shared::{terrain::TerrainState, netty::Packet, listing::GameListing, saves::User, player::{PlayerData, Inventory}, object::{Object, ObjectType}}, ldtk::LDtkMap, assets::{MapAssets, FontAssets, AnimatorAssets, UIAssets, ObjectAssets, ItemAssets, NPCAssets}, consts::{UI_TEXT, PLAYER_CHARACTERS, UI_IMG, FRONT_OBJECTS, BACKGROUND}};

use super::{Netty, ui::{UIManager, UIClickable, UIClickAction}, Disk, chat::ChatMessage, Chat};

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
    /// Chunks that need to be unloaded
    chunks_to_unload: Vec<(isize, isize)>,
    /// Players to spawn in and load
    players_to_spawn: Vec<(User, GamePosition)>,
    /// Players to unload
    players_to_despawn: Vec<User>,
    /// Chunks that are currently loaded
    loaded_chunks: Vec<(isize, isize)>,
    /// Is this server owned by the active player?
    owns_server: bool,
    /// The state of the pause menu
    pause_menu: MenuState,
    /// A list of players that have location changes and their new locations
    players_to_move: HashMap<User, GamePosition>,
    /// Objects that need to be spawned into the bevy world before usage
    queued_objects: Vec<Object>,
    /// Objects that need to be changed in some way
    objects_to_update: Vec<Object>,
    /// Objects that need to be removed
    objects_to_remove: Vec<Uuid>,
    /// Should do an action if the player's selected item supports one
    waiting_for_action: bool,
    /// Data for all chunks that have been modified
    chunk_data: HashMap<(isize, isize), Vec<(usize, usize, TerrainState)>>,
    /// Chunks waiting to be rerendered using `Self::chunk_data`
    waiting_for_update: Vec<(isize, isize)>
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
            chunks_to_unload: vec![],
            players_to_spawn: vec![],
            players_to_despawn: vec![],
            loaded_chunks: vec![],
            owns_server: false,
            pause_menu: MenuState::Closed,
            players_to_move: HashMap::default(),
            queued_objects: vec![],
            objects_to_update: vec![],
            objects_to_remove: vec![],
            waiting_for_action: false,
            chunk_data: HashMap::default(),
            waiting_for_update: vec![],
        }
    }
    pub fn reset(&mut self) {
        *self = Reality::init();
    }
    pub fn queue_action(&mut self) {
        self.waiting_for_action = true;
    }
    pub fn spawn_object(&mut self, object: Object) {
        self.queued_objects.push(object);
    }
    pub fn update_object(&mut self, object: Object) {
        self.objects_to_update.push(object);
    }
    pub fn remove_object(&mut self, object: Uuid) {
        self.objects_to_remove.push(object);
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
        a
    }
    pub fn queue_chat(&mut self, msg: ChatMessage) {
        self.chat_messages.push(msg);
    }
    pub fn set_player_position(&mut self, position: GamePosition) {
        self.player_position = position;
        // load visible world
        const ENV_WIDTH: f64 = 1920.0;
        const ENV_HEIGHT: f64 = 1088.0;
        // Get the player's chunk
        let tile_x = (self.player_position.x / ENV_WIDTH).round() as isize;
        let tile_y = (self.player_position.y / ENV_HEIGHT).round() as isize;
        
        // Add chunks that should be loaded
        self.chunks_to_load.push((tile_x, tile_y));
        self.chunks_to_load.push((tile_x, tile_y + 1));
        self.chunks_to_load.push((tile_x, tile_y - 1));
        self.chunks_to_load.push((tile_x + 1, tile_y));
        self.chunks_to_load.push((tile_x - 1, tile_y));
        self.chunks_to_load.push((tile_x + 1, tile_y + 1));
        self.chunks_to_load.push((tile_x + 1, tile_y - 1));
        self.chunks_to_load.push((tile_x - 1, tile_y + 1));
        self.chunks_to_load.push((tile_x - 1, tile_y - 1));

        // Grab all loaded chunks
        self.chunks_to_unload.append(&mut self.loaded_chunks.clone());
        self.chunks_to_unload.sort();
        // Mark every chunk that isn't about to be loaded to unload
        for chunk in &self.chunks_to_load {
            if let Ok(index) = self.chunks_to_unload.binary_search(chunk) {
                self.chunks_to_unload.remove(index);
            }
        }
    }
    pub fn set_ownership(&mut self, ownership: bool) {
        self.owns_server = ownership;
    }
    /// Add brand new chunk data for a not seen before chunk
    pub fn add_chunk(&mut self, chunk_position: (isize, isize), chunk_data: Vec<(usize, usize, TerrainState)>) {
        self.chunk_data.insert((chunk_position.0, chunk_position.1), chunk_data);
        self.waiting_for_update.push(chunk_position);
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
    pub fn system_render_waiting_chunks(
        mut commands: Commands,
        mut selfs: ResMut<Reality>,
        mut existing_tiles: Query<(Entity, &Tile)>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut maps: ResMut<Assets<LDtkMap>>,
        target_maps: Res<MapAssets>,
    ) {
        let map = maps.get_mut(target_maps.core.clone()).unwrap();
        
        for chunk in &selfs.waiting_for_update {
            if let Some(data) = selfs.chunk_data.get(chunk) {
                for (tile_x, tile_y, tilestate) in data {
                    existing_tiles.for_each_mut(|(entity, tile)| {
                        if
                            tile.chunk == *chunk &&
                            tile.position.0 == *tile_x &&
                            tile.position.1 == *tile_y
                        {
                            let tileset = map.tilesets.get(&(tilestate.tileset as i64)).unwrap();
                            let mut tileset_definition = None;
                            for tileset in &map.project.defs.tilesets {
                                if tileset.uid == tilestate.tileset as i64 {
                                    tileset_definition = Some(tileset);
                                }
                            }
                            let tileset_definition = tileset_definition.unwrap();
                            let texture_atlas = TextureAtlas::from_grid(
                                tileset.clone(),
                                Vec2::from((tileset_definition.tile_grid_size as f32, tileset_definition.tile_grid_size as f32)),
                                tileset_definition.c_hei as usize, tileset_definition.c_wid as usize
                            );
                            let atlas_handle = texture_atlases.add(texture_atlas);
                            commands.entity(entity).despawn();
                            commands.spawn_bundle(SpriteSheetBundle {
                                transform: Transform::from_xyz(
                                    (-1920.0 / 2.0) + (*tile_x as f32 * 64.0) + 32.0 + (1920.0 * chunk.0 as f32),
                                    0.0,
                                    BACKGROUND
                                ),
                                texture_atlas: atlas_handle.clone(),
                                sprite: TextureAtlasSprite::new(tilestate.tile),
                                ..default()
                            }).insert(Tile {
                                chunk: *chunk,
                                position: (*tile_x, *tile_y),
                                sprite: (tilestate.tileset, tilestate.tile)
                            });
                        }
                    });
                }
            }
            else {
                warn!("Unable to find data for a chunk queued for rendering");
            }
        }
        selfs.waiting_for_update.clear();
    }
    pub fn system_remove_objects(
        mut commands: Commands,
        mut selfs: ResMut<Reality>,
        mut objects: Query<(Entity, &Object)>
    ) {
        for removable in &selfs.objects_to_remove {
            objects.for_each_mut(|(entity, object)| {
                if object.uuid == *removable {
                    commands.entity(entity).despawn();
                }
            });
        }
        selfs.objects_to_remove.clear();
    }
    pub fn system_update_objects(
        mut selfs: ResMut<Reality>,
        mut objects: Query<(&mut Transform, &mut Object)>
    ) {
        for updateable in &selfs.objects_to_update {
            objects.for_each_mut(|(mut transform, mut object)| {
                if object.uuid == updateable.uuid {
                    object.update(updateable.clone());
                    transform.translation.x = object.pos.x as f32;
                    transform.translation.y = object.pos.y as f32;
                }
            });
        }
        selfs.objects_to_update.clear();
    }
    pub fn system_spawn_objects(
        mut selfs: ResMut<Reality>,
        obj_assets: Res<ObjectAssets>,
        item_assets: Res<ItemAssets>,
        npc_assets: Res<NPCAssets>,
        mut commands: Commands
    ) {
        for object in &selfs.queued_objects {
            match &object.rep {
                ObjectType::Tree => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: obj_assets.tree.clone(),
                        transform: Transform::from_xyz(object.pos.x as f32, object.pos.y as f32, FRONT_OBJECTS),
                        ..default()
                    }).insert(object.clone());
                }
                ObjectType::GroundItem(item) => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: item_assets.pick_from_item(*item),
                        transform: Transform::from_xyz(object.pos.x as f32, object.pos.y as f32, FRONT_OBJECTS),
                        ..default()
                    }).insert(object.clone());
                }
                ObjectType::NPC(_who) => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: npc_assets.not_animated.clone(),
                        transform: Transform::from_xyz(object.pos.x as f32, object.pos.y as f32, PLAYER_CHARACTERS),
                        ..default()
                    }).insert(object.clone());
                }
            }
        }
        selfs.queued_objects.clear();
    }
    pub fn system_spawn_hotbar(
        mut commands: Commands,
        textures: Res<UIAssets>,
        items: Res<ItemAssets>,
        selfs: Res<Reality>
    ) {
        for i in 0..10 {
            commands.spawn_bundle(SpriteBundle {
                texture: textures.slot.clone(),
                transform: Transform::from_xyz(i as f32 * 64.0 - (64.0 * 5.0), -(1080.0 / 2.0) + 32.0, UI_IMG),
                ..Default::default()
            }).insert(HotbarMarker { location: i, type_: 1 }).insert(UILocked {});
            commands.spawn_bundle(SpriteBundle {
                texture: items.none.clone(),
                transform: Transform::from_xyz(i as f32 * 64.0 - (64.0 * 5.0), -(1080.0 / 2.0) + 32.0, UI_IMG + 0.01),
                ..Default::default()
            }).insert(HotbarMarker { location: i, type_: 3 }).insert(UILocked {});
        }
        commands.spawn_bundle(SpriteBundle {
            texture: textures.selected.clone(),
            transform: Transform::from_xyz(
                    selfs.player.inventory.selected_slot as f32 * 64.0 - (64.0 * 5.0),
                    -(1080.0 / 2.0) + 32.0,
                    UI_IMG + 0.02
                ),
                ..Default::default()
        }).insert(HotbarMarker { location: selfs.player.inventory.selected_slot, type_: 2 }).insert(UILocked {});
    }
    pub fn system_update_hotbar(
        selfs: Res<Reality>,
        textures: Res<ItemAssets>,
        mut query: Query<(&HotbarMarker, &mut Handle<Image>)>
    ) {
        query.for_each_mut(|(marker, mut texture)| {
            if marker.type_ == 3 {
                texture.set(Box::new(textures.pick_from_item(selfs.player.inventory.hotbar[marker.location]))).unwrap();
            }
        });
    }
    pub fn system_position_hotbar(
        mut query: Query<(&mut Transform, &HotbarMarker)>
    ) {
        query.for_each_mut(|(mut location, spot)| {
            location.translation.x = spot.location as f32 * 64.0 - (64.0 * 5.0);
            location.translation.y = -(1080.0 / 2.0) + 32.0;
        });
    }
    pub fn system_scroll_hotbar(
        mut query: Query<&mut HotbarMarker>,
        mut scroll: EventReader<MouseWheel>,
        mut selfs: ResMut<Reality>
    ) {
        for event in scroll.iter() {
            match event.unit {
                MouseScrollUnit::Line => {
                    if event.y.is_sign_negative() {
                        query.for_each_mut(|mut mark| {
                            if mark.type_ == 2 {
                                if mark.location < 9 {
                                    mark.location += 1;
                                }
                                else {
                                    mark.location = 0;
                                }
                                selfs.player.inventory.selected_slot = mark.location;
                            }
                        });
                    }
                    else {
                        query.for_each_mut(|mut mark| {
                            if mark.type_ == 2 {
                                if mark.location > 0 {
                                    mark.location -= 1;
                                }
                                else {
                                    mark.location = 9;
                                }
                                selfs.player.inventory.selected_slot = mark.location;
                            }
                        });
                    }
                }
                MouseScrollUnit::Pixel => {
                    // We don't use trackpads to scroll the hotbar. Consider supporting this in the future.
                }
            }
        }
    }
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
                let a = maps.get_mut(target_maps.core.clone()).unwrap();
                crate::ldtk::load_chunk(chunk, a, &mut texture_atlases, fonts.clone(), &mut commands);
                selfs.loaded_chunks.push(chunk);
                // sorted order is needed for .binary_search() used in `system_chunk_unloader`
                selfs.loaded_chunks.sort();
            }
        }
        selfs.chunks_to_load.clear();
    }
    pub fn system_chunk_unloader(
        mut selfs: ResMut<Reality>,
        mut commands: Commands,
        mut query: Query<(Entity, &Tile)>
    ) {
        for chunk in selfs.chunks_to_unload.clone() {
            // remove chunk from loaded chunks list, if found
            if let Ok(index) = selfs.loaded_chunks.binary_search(&chunk) {
                selfs.loaded_chunks.remove(index);
            }
            else {
                warn!("A chunk was queued to be removed but isn't loaded");
            }
            query.for_each_mut(|(entity, tile)| {
                if tile.chunk == chunk {
                    commands.entity(entity).despawn();
                }
            });
        }
        selfs.chunks_to_unload.clear();
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
                    texture: assets.not_animated.clone(),
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
        disk: Res<Disk>,
        keyboard: Res<Input<KeyCode>>
    ) {
        if keyboard.just_pressed(disk.control_config().close_menu) {
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
        mut chat: ResMut<Chat>,
        disk: Res<Disk>,
        mut queries: ParamSet<(
            Query<&Tile>,
            Query<&Object>,
        )>
    ) {
        let ctrls = disk.control_config();
        if !chat.is_open() && selfs.pause_menu == MenuState::Closed && keyboard.just_pressed(ctrls.open_chat) {
            chat.queue_open();
            return;
        }
        if chat.is_open() && keyboard.just_pressed(ctrls.close_menu) {
            chat.escape_close();
            return;
        }
        if keyboard.any_pressed([ctrls.move_up, ctrls.move_down, ctrls.move_left, ctrls.move_right]) && selfs.pause_menu == MenuState::Closed && !chat.is_open() {
            let centered_chunk = (
                ((selfs.player_position.x + (1920.0 / 2.0)) / 1920.0).floor() as isize,
                ((selfs.player_position.y + (1088.0 / 2.0)) / 1088.0).floor() as isize
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
            queries.p0().for_each(|tile| {
                if needed_chunks.contains(&tile.chunk) {
                    for (chunk, n_tile) in &needed_pairs {
                        if tile.chunk == *chunk && tile.position == *n_tile {
                            pulled_tiles.push(tile.clone());
                        }
                    }
                }
            });
            let mut objects = vec![];
            queries.p1().for_each(|object| {
                objects.push(object.clone());
            });
            let mut had_movement = false;
            let mut new_pos = selfs.player_position;
            // move
            if keyboard.pressed(ctrls.move_up) {
                new_pos.y += 4.0;
                if  !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.x, new_pos.y)) &&
                    !calc_player_against_objects(objects.as_slice(), (new_pos.x, new_pos.y)) {
                    had_movement = true;
                }
                else {
                    new_pos.y -= 4.0;
                }
            }
            if keyboard.pressed(ctrls.move_down) {
                new_pos.y -= 4.0;
                if  !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.x, new_pos.y)) &&
                    !calc_player_against_objects(objects.as_slice(), (new_pos.x, new_pos.y)){
                    had_movement = true;
                }
                else {
                    new_pos.y += 4.0;
                }
            }
            if keyboard.pressed(ctrls.move_left) {
                new_pos.x -= 4.0;
                if  !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.x, new_pos.y)) &&
                    !calc_player_against_objects(objects.as_slice(), (new_pos.x, new_pos.y)) {
                    had_movement = true;
                }
                else {
                    new_pos.x += 4.0;
                }
            }
            if keyboard.pressed(ctrls.move_right) {
                new_pos.x += 4.0;
                if  !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.x, new_pos.y)) &&
                    !calc_player_against_objects(objects.as_slice(), (new_pos.x, new_pos.y)) {
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
                    location: (-150.0, 110.0),
                    size: (300.0, 55.0),
                    removed_on_use: false,
                    tag: None
                });
                uiman.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("InvitePlayer")),
                    location: (-150.0, 55.0),
                    size: (300.0, 55.0),
                    removed_on_use: false,
                    tag: None
                });
                uiman.add_ui(UIClickable {
                    action: UIClickAction::ChangeScene(String::from("Settings")),
                    location: (-150.0, 0.0),
                    size: (300.0, 55.0),
                    removed_on_use: false,
                    tag: None
                });
                uiman.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("LeaveGame")),
                    location: (-150.0, -55.0),
                    size: (300.0, 55.0),
                    removed_on_use: false,
                    tag: None
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
                        component_id => {
                            warn!("Pause menu component has unkown type id {component_id}");
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
                    username: tb.grab_buffer().split('#').next().unwrap().to_string(),
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
        mut queries: ParamSet<(
            Query<&mut Transform, With<Camera>>,
            Query<&mut Transform, With<UILocked>>
        )>
    ) {
        queries.p0().for_each_mut(|mut campos| {
            campos.translation.x = selfs.player_position.x as f32;
            campos.translation.y = selfs.player_position.y as f32;
            if selfs.loaded_chunks.is_empty() {
                campos.translation.x = 0.0;
                campos.translation.y = 0.0;
            }
        });
        queries.p1().for_each_mut(|mut transform| {
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
                    removed_on_use: false,
                    tag: None
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
fn calc_player_against_tiles(tiles: &[Tile], player: (f64, f64)) -> bool {
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

fn calc_player_against_objects(objects: &[Object], player: (f64, f64)) -> bool {
    for object in objects {
        if let Some(obj_size) = object.rep.collider() {
            let obj_left = object.pos.x - (obj_size.0 / 2.0);
            let obj_right = obj_left + obj_size.0;
            let obj_bottom = object.pos.y - (obj_size.1 / 2.0);
            let obj_top = obj_bottom + obj_size.1;
            let player_left = player.0 - 32.0;
            let player_right = player.0 + 32.0;
            let player_top = player.1 + 32.0;
            let player_bottom = player.1 - 32.0;
            if  player_right > obj_left &&
                player_left < obj_right &&
                player_bottom < obj_top &&
                player_top > obj_bottom
            {
                return true;
            }
        }
    }
    false
}
