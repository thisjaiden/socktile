use bevy::{prelude::*, render::camera::Camera, utils::HashMap, input::mouse::{MouseWheel, MouseScrollUnit}};
use bevy_prototype_debug_lines::DebugLines;
use uuid::Uuid;

use crate::{components::{GamePosition, ldtk::{PlayerMarker, TileMarker, Tile}, PauseMenuMarker, UILocked, HotbarMarker}, shared::{netty::Packet, listing::GameListing, saves::User, player::{PlayerData, Inventory}, object::{Object, ObjectType}}, assets::{FontAssets, AnimatorAssets, UIAssets, ObjectAssets, ItemAssets, NPCAssets, CoreAssets}, consts::{UI_TEXT, PLAYER_CHARACTERS, UI_IMG, FRONT_OBJECTS, CHUNK_WIDTH, CHUNK_HEIGHT, BACKGROUND, CHUNK_SIZE, TERRAIN_DEBUG, DEBUG, PLAYER_HITBOX, PLAYER_DEBUG}, modular_assets::{ModularAssets, TerrainRendering, TransitionType}};
use crate::prelude::*;

use super::{Netty, ui::{UIManager, UIClickable, UIClickAction}, Disk, chat::ChatMessage, Chat};

pub struct Reality {
    /// Player's current position
    player_position: GamePosition,
    /// Once the player is connected this is set to true
    in_valid_world: bool,
    /// Player's data
    player: PlayerData,
    /// Queued chat messages
    chat_messages: Vec<ChatMessage>,
    /// Servers that can be joined
    avalable_servers: Vec<GameListing>,
    push_servers: bool,
    /// Players to spawn in and load
    players_to_spawn: Vec<(User, GamePosition)>,
    /// Players to unload
    players_to_despawn: Vec<User>,
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
    /// Data for all chunks
    chunk_data: HashMap<(isize, isize), Vec<usize>>,
    chunk_status: HashMap<(isize, isize), ChunkStatus>,
}

impl Reality {
    pub fn init() -> Reality {
        Reality {
            player_position: GamePosition::zero(),
            in_valid_world: false,
            player: PlayerData::new(),
            chat_messages: vec![],
            avalable_servers: vec![],
            push_servers: false,
            players_to_spawn: vec![],
            players_to_despawn: vec![],
            owns_server: false,
            pause_menu: MenuState::Closed,
            players_to_move: default(),
            queued_objects: vec![],
            objects_to_update: vec![],
            objects_to_remove: vec![],
            waiting_for_action: false,
            chunk_data: default(),
            chunk_status: default(),
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
    /// Sets the player's position. This funciton also loads and unloads
    /// appropriate chunks around the player's new position.
    pub fn set_player_position(&mut self, position: GamePosition) {
        // Set the player's position
        self.player_position = position;
    }
    pub fn set_ownership(&mut self, ownership: bool) {
        info!("Setting ownership status to {ownership}");
        self.owns_server = ownership;
        self.in_valid_world = true;
    }
    /// Add brand new chunk data for a not seen before chunk
    pub fn add_chunk(&mut self, chunk_position: (isize, isize), chunk_data: Vec<usize>) {
        self.chunk_data.insert((chunk_position.0, chunk_position.1), chunk_data);
        // we should never be sent a chunk we haven't requested and therefore don't have metadata for
        let status = self.chunk_status.get_mut(&chunk_position).unwrap();
        status.downloaded = true;
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
    /// Marks chunks to be rendered, downloaded, and unrendered. This system is essential to
    /// the world loading and collision loading
    pub fn system_mark_chunks(
        mut selfs: ResMut<Reality>
    ) {
        if selfs.is_changed() && selfs.in_valid_world {
            // Chunk sizes in coordinate space
            const ENV_WIDTH: f64 = 1920.0;
            const ENV_HEIGHT: f64 = 1088.0;
            // Get the player's chunk
            let chunk_x = (selfs.player_position.x / ENV_WIDTH).round() as isize;
            let chunk_y = (selfs.player_position.y / ENV_HEIGHT).round() as isize;

            // Add chunks that should be loaded (5x5 around player) for download if they aren't
            // already avalable
            run_matrix_nxn(-2..=2, |x, y| {
                if !selfs.chunk_status.contains_key(&(x + chunk_x, y + chunk_y)) {
                    selfs.chunk_status.insert(
                        (x + chunk_x, y + chunk_y),
                        ChunkStatus {
                            rendered: false,
                            downloaded: false,
                            needs_download_request: true,
                            waiting_to_render: false, // TODO this is error
                            stop_rendering: false,
                            edges_rendered: true
                        }
                    );
                }
            });

            for (chunk, status) in selfs.chunk_status.iter_mut() {
                // mark all chunks that aren't around the player to stop rendering
                if !get_matrix_nxn(-1..=1).contains(&((chunk.0 - chunk_x) as i8, (chunk.1 - chunk_y) as i8)) {
                    if status.rendered {
                        status.stop_rendering = true;
                    }
                }
                else {
                    // mark all chunks that are around the player to start rendering
                    if !status.rendered && status.downloaded {
                        status.waiting_to_render = true;
                    }
                }
            }
        }
    }
    /// Finds every chunk we have metadata for but no actual data, and requests a copy of it.
    pub fn system_chunk_requester(
        mut selfs: ResMut<Reality>,
        mut netty: ResMut<Netty>
    ) {
        for (chunk, status) in selfs.chunk_status.iter_mut() {
            if !status.downloaded {
                netty.say(Packet::RequestChunk(*chunk));
                status.needs_download_request = false;
            }
        }
    }
    pub fn system_chunk_derenderer(
        mut commands: Commands,
        mut selfs: ResMut<Reality>,
        tiles: Query<(Entity, &Tile)>
    ) {
        for (chunk, status) in selfs.chunk_status.iter_mut() {
            if status.stop_rendering && status.rendered {
                info!("Unrendering chunk {:?}", chunk);
                tiles.for_each(|(e, tile)| {
                    if tile.chunk == *chunk {
                        commands.entity(e).despawn();
                    }
                });
                status.stop_rendering = false;
                status.rendered = false;
            }
        }
    }
    pub fn system_render_waiting_chunks(
        mut commands: Commands,
        mut selfs: ResMut<Reality>,
        core: Res<CoreAssets>,
        core_serve: Res<Assets<ModularAssets>>,
        mut atlas_serve: ResMut<Assets<TextureAtlas>>
    ) {
        let mod_assets = core_serve.get(core.core.clone()).unwrap();
        let mut inserts = vec![];
        for (chunk, status) in selfs.chunk_status.iter() {
            if status.waiting_to_render && status.downloaded {
                let mut status = *status;
                status.waiting_to_render = false;
                status.rendered = true;
                let stuff: Vec<usize>;
                if !status.edges_rendered {
                    status.edges_rendered = true;
                    stuff = CHUNK_EDGES.to_vec();
                }
                else {
                    stuff = (0..CHUNK_SIZE).collect();
                }
                inserts.push((*chunk, status));
                let top = inserts.len() - 1;
                if let Some(data) = selfs.chunk_data.get(chunk) {
                    for index in stuff {
                        let tile_x = index % CHUNK_WIDTH;
                        let tile_y = index / CHUNK_WIDTH;
                        let pot_rendering = get_tile_rendering(tile_x, tile_y, mod_assets, data, &selfs, chunk, &mut inserts, top);
                        if let Some(rendering) = pot_rendering {
                            match rendering.0 {
                                TerrainRendering::Sprite(img) => {
                                    commands.spawn_bundle(SpriteBundle {
                                        transform: Transform::from_xyz(
                                            (-1920.0 / 2.0) + (tile_x as f32 * 64.0) + 32.0 + (1920.0 * chunk.0 as f32),
                                            (-1080.0 / 2.0) + (tile_y as f32 * 64.0) - 32.0 + (1088.0 * chunk.1 as f32),
                                            BACKGROUND
                                        ),
                                        texture: img,
                                        ..default()
                                    })
                                    .insert(Tile {
                                        chunk: *chunk,
                                        position: (tile_x, tile_y),
                                        transition_type: rendering.1
                                    });
                                },
                                TerrainRendering::SpriteSheet(img, width, height, loc) => {
                                    let sprite = TextureAtlasSprite {
                                        index: loc,
                                        ..default()
                                    };
                                    let new_atlas = TextureAtlas::from_grid(
                                        img,
                                        Vec2::new(64.0, 64.0),
                                        width, height
                                    );
                                    let atlas_handle = atlas_serve.add(new_atlas);
                                    commands.spawn_bundle(SpriteSheetBundle {
                                        transform: Transform::from_xyz(
                                            (-1920.0 / 2.0) + (tile_x as f32 * 64.0) + 32.0 + (1920.0 * chunk.0 as f32),
                                            (-1080.0 / 2.0) + (tile_y as f32 * 64.0) - 32.0 + (1088.0 * chunk.1 as f32),
                                            BACKGROUND
                                        ),
                                        sprite,
                                        texture_atlas: atlas_handle,
                                        ..default()
                                    })
                                    .insert(Tile {
                                        chunk: *chunk,
                                        position: (tile_x, tile_y),
                                        transition_type: rendering.1
                                    });
                                },
                                TerrainRendering::AnimatedSprite(imgs, animation) => {
                                    todo!()
                                },
                                TerrainRendering::AnimatedSpriteSheet(_, _) => {
                                    todo!()
                                }
                            }
                        }
                    }
                }
                else {
                    warn!("Unable to find data for a chunk queued for rendering");
                }
            }
        }
        for (loc, dta) in inserts {
            selfs.chunk_status.insert(loc, dta);
        }
    }
    pub fn system_rerender_edges(
        mut selfs: ResMut<Reality>
    ) {
        let mut chunks_to_rerender = vec![];
        for (chunk, status) in selfs.chunk_status.iter() {
            if !status.edges_rendered && status.rendered {
                let mut should_rerender = true;
                run_matrix_nxn(-1..=1, |x, y| {
                    if !selfs.chunk_data.contains_key(&(chunk.0 + x, chunk.1 + y)) {
                        should_rerender = false;
                    }
                });
                if should_rerender {
                    chunks_to_rerender.push(*chunk);
                }
            }
        }
        for chunk in chunks_to_rerender {
            info!("Marking chunk {:?} for rerendering", chunk);
            let data = selfs.chunk_status.get_mut(&chunk).unwrap();
            data.waiting_to_render = true;
        }
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
                        transform: Transform {
                            translation: Vec3::new(
                                object.pos.x as f32,
                                object.pos.y as f32,
                                FRONT_OBJECTS
                            ),
                            rotation: Quat::default(),
                            scale: Vec3::new(0.1, 0.1, 1.0)
                        },
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
                ObjectType::Npc(_who) => {
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
            // TODO: BUG: ERROR GRABBING COLLISION IN OTHER CHUNKS
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
                            pulled_tiles.push(*tile);
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
                    action: UIClickAction::ClosePauseMenu,
                    location: (-150.0, 110.0),
                    size: (300.0, 55.0),
                    removed_on_use: false,
                    tag: None
                });
                uiman.add_ui(UIClickable {
                    action: UIClickAction::InvitePlayer,
                    location: (-150.0, 55.0),
                    size: (300.0, 55.0),
                    removed_on_use: false,
                    tag: None
                });
                uiman.add_ui(UIClickable {
                    action: UIClickAction::OpenSettings,
                    location: (-150.0, 0.0),
                    size: (300.0, 55.0),
                    removed_on_use: false,
                    tag: None
                });
                uiman.add_ui(UIClickable {
                    action: UIClickAction::DisconnectFromWorld,
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
            if selfs.chunk_status.is_empty() {
                campos.translation.x = 0.0;
                campos.translation.y = 0.0;
            }
        });
        queries.p1().for_each_mut(|mut transform| {
            transform.translation.x += selfs.player_position.x as f32;
            transform.translation.y += selfs.player_position.y as f32;
        });
    }
    pub fn system_player_debug_lines(
        selfs: Res<Reality>,
        mut lines: ResMut<DebugLines>
    ) {
        if PLAYER_DEBUG {
            lines.line_colored(
                Vec3::new(
                    (selfs.player_position.x - (PLAYER_HITBOX.0 / 2.0)) as f32,
                    (selfs.player_position.y - (PLAYER_HITBOX.1 / 2.0)) as f32,
                    DEBUG
                ),
                Vec3::new(
                    (selfs.player_position.x + (PLAYER_HITBOX.0 / 2.0)) as f32,
                    (selfs.player_position.y + (PLAYER_HITBOX.1 / 2.0)) as f32,
                    DEBUG
                ),
                0.0,
                Color::ORANGE
            );
        }
    }
    pub fn system_hitbox_debug_lines(
        mut lines: ResMut<DebugLines>,
        tiles: Query<&Tile>
    ) {
        if TERRAIN_DEBUG {
            tiles.for_each(|tile| {
                let dta = tile.transition_type.collider_dimensions();
                for collider in dta {
                    let true_x = collider.0 + (tile.position.0 as f64 * 64.0) + (tile.chunk.0 as f64 * 1920.0) - (1920.0 / 2.0);
                    let true_y = collider.1 + (tile.position.1 as f64 * 64.0) + (tile.chunk.1 as f64 * 1088.0) - (1088.0 / 2.0) - 66.0;
                    lines.line_colored(
                        Vec3::new(true_x as f32, true_y as f32, DEBUG),
                        Vec3::new((true_x + collider.2) as f32, true_y as f32, DEBUG),
                        0.0,
                        Color::RED
                    );
                    lines.line_colored(
                        Vec3::new(true_x as f32, true_y as f32, DEBUG),
                        Vec3::new(true_x as f32, (true_y + collider.3) as f32, DEBUG),
                        0.0,
                        Color::RED
                    );
                    lines.line_colored(
                        Vec3::new((true_x + collider.2) as f32, true_y as f32, DEBUG),
                        Vec3::new((true_x + collider.2) as f32, (true_y + collider.3) as f32, DEBUG),
                        0.0,
                        Color::RED
                    );
                    lines.line_colored(
                        Vec3::new(true_x as f32, (true_y + collider.3) as f32, DEBUG),
                        Vec3::new((true_x + collider.2) as f32, (true_y + collider.3) as f32, DEBUG),
                        0.0,
                        Color::RED
                    );
                }
                
            });
        }
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
                            horizontal: HorizontalAlign::Left
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
        if tile.transition_type.collides(player, offset_x, offset_y) {
            return true;
        }
    }
    false
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ChunkStatus {
    rendered: bool,
    needs_download_request: bool,
    downloaded: bool,
    waiting_to_render: bool,
    stop_rendering: bool,
    edges_rendered: bool
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

const CHUNK_EDGES: [usize; (CHUNK_WIDTH * 2) + ((CHUNK_HEIGHT - 2) * 2)] = [
    // bottom edge
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29,
    // top edge
    480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494,
    495, 496, 497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509,
    // left edge
    30, 60, 90, 120, 150, 180, 210, 240, 280, 300, 330, 360, 390, 420, 450,
    // right edge
    59, 89, 119, 149, 179, 209, 239, 269, 299, 329, 259, 289, 419, 449, 479
];

fn get_tile_rendering(
    tile_x: usize,
    tile_y: usize,
    mod_assets: &ModularAssets,
    data: &[usize],
    selfs: &ResMut<Reality>,
    chunk: &(isize, isize),
    inserts: &mut Vec<((isize, isize), ChunkStatus)>,
    top: usize,
) -> Option<(TerrainRendering, TransitionType)> {
    let rendering;
    if tile_x > 0 {
        if tile_x < CHUNK_WIDTH - 1 {
            if tile_y > 0 {
                if tile_y < CHUNK_HEIGHT - 1 {
                    // all tiles are within this chunk
                    rendering = mod_assets.get_tile([
                        data[tile_x - 1 + ((tile_y + 1) * CHUNK_WIDTH)],
                        data[tile_x + ((tile_y + 1) * CHUNK_WIDTH)],
                        data[tile_x + 1 + ((tile_y + 1) * CHUNK_WIDTH)],

                        data[tile_x - 1 + (tile_y * CHUNK_WIDTH)],
                        data[tile_x + (tile_y * CHUNK_WIDTH)],
                        data[tile_x + 1 + (tile_y * CHUNK_WIDTH)],

                        data[tile_x - 1 + ((tile_y - 1) * CHUNK_WIDTH)],
                        data[tile_x + ((tile_y - 1) * CHUNK_WIDTH)],
                        data[tile_x + 1 + ((tile_y - 1) * CHUNK_WIDTH)]
                    ]);
                }
                else {
                    // some y tiles are one chunk above
                    let pot_data_up = selfs.chunk_data.get(&(chunk.0, chunk.1 + 1));
                    if let Some(data_up) = pot_data_up {
                        rendering = mod_assets.get_tile([
                            data_up[tile_x - 1],
                            data_up[tile_x],
                            data_up[tile_x + 1],

                            data[tile_x - 1 + (tile_y * CHUNK_WIDTH)],
                            data[tile_x + (tile_y * CHUNK_WIDTH)],
                            data[tile_x + 1 + (tile_y * CHUNK_WIDTH)],

                            data[tile_x - 1 + ((tile_y - 1) * CHUNK_WIDTH)],
                            data[tile_x + ((tile_y - 1) * CHUNK_WIDTH)],
                            data[tile_x + 1 + ((tile_y - 1) * CHUNK_WIDTH)]
                        ]);
                    }
                    else {
                        // we don't have that chunk in memory, so don't render this tile.
                        inserts[top].1.edges_rendered = false;
                        return None;
                    }
                }
            }
            else {
                // some y tiles are one chunk below
                let pot_data_down = selfs.chunk_data.get(&(chunk.0, chunk.1 - 1));
                if let Some(data_down) = pot_data_down {
                    rendering = mod_assets.get_tile([
                        data[tile_x - 1 + ((tile_y + 1) * CHUNK_WIDTH)],
                        data[tile_x + ((tile_y + 1) * CHUNK_WIDTH)],
                        data[tile_x + 1 + ((tile_y + 1) * CHUNK_WIDTH)],

                        data[tile_x - 1 + (tile_y * CHUNK_WIDTH)],
                        data[tile_x + (tile_y * CHUNK_WIDTH)],
                        data[tile_x + 1 + (tile_y * CHUNK_WIDTH)],

                        data_down[tile_x - 1],
                        data_down[tile_x],
                        data_down[tile_x + 1]
                    ]);
                }
                else {
                    // we don't have that chunk in memory, so don't render this tile.
                    inserts[top].1.edges_rendered = false;
                    return None;
                }
            }
        }
        else if tile_y > 0 {
            if tile_y < CHUNK_HEIGHT - 1 {
                // some x tiles are one chunk right
                let pot_data_right = selfs.chunk_data.get(&(chunk.0 + 1, chunk.1));
                if let Some(data_right) = pot_data_right {
                    rendering = mod_assets.get_tile([
                        data[tile_x - 1 + ((tile_y + 1) * CHUNK_WIDTH)],
                        data[tile_x + ((tile_y + 1) * CHUNK_WIDTH)],
                        data_right[((tile_y + 1) * CHUNK_WIDTH)],

                        data[tile_x - 1 + (tile_y * CHUNK_WIDTH)],
                        data[tile_x + (tile_y * CHUNK_WIDTH)],
                        data_right[(tile_y * CHUNK_WIDTH)],

                        data[tile_x - 1 + ((tile_y - 1) * CHUNK_WIDTH)],
                        data[tile_x + ((tile_y - 1) * CHUNK_WIDTH)],
                        data_right[((tile_y - 1) * CHUNK_WIDTH)]
                    ]);
                }
                else {
                    // we don't have one of the chunks we need, so don't render this tile.
                    inserts[top].1.edges_rendered = false;
                    return None;
                }
            }
            else {
                // some x tiles are one chunk right AND
                // some y tiles are one chunk above AND
                // one tile is one chunk above and right
                let pot_data_right = selfs.chunk_data.get(&(chunk.0 + 1, chunk.1));
                let pot_data_up = selfs.chunk_data.get(&(chunk.0, chunk.1 + 1));
                let pot_data_up_right = selfs.chunk_data.get(&(chunk.0 + 1, chunk.1 + 1));
                if pot_data_right.is_none() || pot_data_up.is_none() || pot_data_up_right.is_none() {
                    // we don't have one of the chunks we need, so don't render this tile.
                    inserts[top].1.edges_rendered = false;
                    return None;
                }
                let data_right = pot_data_right.unwrap();
                let data_up = pot_data_up.unwrap();
                let data_up_right = pot_data_up_right.unwrap();
                rendering = mod_assets.get_tile([
                    data_up[tile_x - 1],
                    data_up[tile_x],
                    data_up_right[0],
                    
                    data[tile_x - 1 + (tile_y * CHUNK_WIDTH)],
                    data[tile_x + (tile_y * CHUNK_WIDTH)],
                    data_right[(tile_y * CHUNK_WIDTH)],
                    
                    data[tile_x - 1 + ((tile_y - 1) * CHUNK_WIDTH)],
                    data[tile_x + ((tile_y - 1) * CHUNK_WIDTH)],
                    data_right[((tile_y - 1) * CHUNK_WIDTH)]
                ]);
            }
        }
        else {
            // some x tiles are one chunk right AND
            // some y tiles are one chunk below AND
            // one tile is below and right
            let pot_data_right = selfs.chunk_data.get(&(chunk.0 + 1, chunk.1));
            let pot_data_down = selfs.chunk_data.get(&(chunk.0, chunk.1 - 1));
            let pot_data_down_right = selfs.chunk_data.get(&(chunk.0 + 1, chunk.1 - 1));
            if pot_data_right.is_none() || pot_data_down.is_none() || pot_data_down_right.is_none() {
                // we don't have one of the chunks we need, so don't render this tile.
                inserts[top].1.edges_rendered = false;
                return None;
            }
            let data_right = pot_data_right.unwrap();
            let data_down = pot_data_down.unwrap();
            let data_down_right = pot_data_down_right.unwrap();
            rendering = mod_assets.get_tile([
                data[tile_x - 1 + ((tile_y + 1) * CHUNK_WIDTH)],
                data[tile_x + ((tile_y + 1) * CHUNK_WIDTH)],
                data_right[(tile_y * CHUNK_WIDTH)],

                data[tile_x - 1 + (tile_y * CHUNK_WIDTH)],
                data[tile_x + (tile_y * CHUNK_WIDTH)],
                data_right[(tile_y * CHUNK_WIDTH)],

                data_down[tile_x - 1 + (CHUNK_WIDTH * (CHUNK_HEIGHT - 1))],
                data_down[tile_x + (CHUNK_WIDTH * (CHUNK_HEIGHT - 1))],
                data_down_right[CHUNK_WIDTH * (CHUNK_HEIGHT - 1)]
            ]);
        }
    }
    else if tile_y > 0 {
        if tile_y < CHUNK_HEIGHT - 1 {
            // some x tiles are one chunk left
            let pot_data_left = selfs.chunk_data.get(&(chunk.0 - 1, chunk.1));
            if let Some(data_left) = pot_data_left {
                rendering = mod_assets.get_tile([
                    data_left[CHUNK_WIDTH - 1 + ((tile_y + 1) * CHUNK_WIDTH)],
                    data[tile_x + ((tile_y + 1) * CHUNK_WIDTH)],
                    data[tile_x + 1 + ((tile_y + 1) * CHUNK_WIDTH)],

                    data_left[CHUNK_WIDTH - 1 + (tile_y * CHUNK_WIDTH)],
                    data[tile_x + (tile_y * CHUNK_WIDTH)],
                    data[tile_x + 1 + (tile_y * CHUNK_WIDTH)],

                    data_left[CHUNK_WIDTH - 1 + ((tile_y - 1) * CHUNK_WIDTH)],
                    data[tile_x + ((tile_y - 1) * CHUNK_WIDTH)],
                    data[tile_x + 1 + ((tile_y - 1) * CHUNK_WIDTH)]
                ]);
            }
            else {
                // we don't have one of the chunks we need, so don't render this tile.
                inserts[top].1.edges_rendered = false;
                return None;
            }
        }
        else {
            // some x tiles are one chunk left AND
            // some y tiles are one chunk above AND
            // one tile is above and left
            let pot_data_left = selfs.chunk_data.get(&(chunk.0 - 1, chunk.1));
            let pot_data_up = selfs.chunk_data.get(&(chunk.0, chunk.1 + 1));
            let pot_data_up_left = selfs.chunk_data.get(&(chunk.0 - 1, chunk.1 + 1));
            if pot_data_left.is_none() || pot_data_up.is_none() || pot_data_up_left.is_none() {
                // we don't have one of the chunks we need, so don't render this tile.
                inserts[top].1.edges_rendered = false;
                return None;
            }
            let data_left = pot_data_left.unwrap();
            let data_up = pot_data_up.unwrap();
            let data_up_left = pot_data_up_left.unwrap();
            rendering = mod_assets.get_tile([
                data_up_left[CHUNK_WIDTH - 1],
                data_up[tile_x],
                data_up[tile_x + 1],

                data_left[CHUNK_WIDTH - 1 + (tile_y * CHUNK_WIDTH)],
                data[tile_x + (tile_y * CHUNK_WIDTH)],
                data[tile_x + 1 + (tile_y * CHUNK_WIDTH)],

                data_left[CHUNK_WIDTH - 1 + ((tile_y - 1) * CHUNK_WIDTH)],
                data[tile_x + ((tile_y - 1) * CHUNK_WIDTH)],
                data[tile_x + 1 + ((tile_y - 1) * CHUNK_WIDTH)]
            ]);
        }
    }
    else {
        // some x tiles are one chunk left AND
        // some y tiles are one chunk below
        // one tile is below and left
        let pot_data_left = selfs.chunk_data.get(&(chunk.0 - 1, chunk.1));
        let pot_data_down = selfs.chunk_data.get(&(chunk.0, chunk.1 - 1));
        let pot_data_down_left = selfs.chunk_data.get(&(chunk.0 - 1, chunk.1 - 1));
        if pot_data_left.is_none() || pot_data_down.is_none() || pot_data_down_left.is_none() {
            // we don't have one of the chunks we need, so don't render this tile.
            inserts[top].1.edges_rendered = false;
            return None;
        }
        let data_left = pot_data_left.unwrap();
        let data_down = pot_data_down.unwrap();
        let data_down_left = pot_data_down_left.unwrap();
        rendering = mod_assets.get_tile([
            data_left[CHUNK_WIDTH - 1 + ((tile_y + 1) * CHUNK_WIDTH)],
            data[tile_x + ((tile_y + 1) * CHUNK_WIDTH)],
            data[tile_x + 1 + ((tile_y + 1) * CHUNK_WIDTH)],

            data_left[CHUNK_WIDTH - 1 + (tile_y * CHUNK_WIDTH)],
            data[tile_x + (tile_y * CHUNK_WIDTH)],
            data[tile_x + 1 + (tile_y * CHUNK_WIDTH)],

            data_down_left[CHUNK_SIZE - 1],
            data_down[tile_x + (CHUNK_WIDTH * (CHUNK_HEIGHT - 1))],
            data_down[tile_x + 1 + (CHUNK_WIDTH * (CHUNK_HEIGHT - 1))]
        ]);
    }
    Some(rendering)
}
