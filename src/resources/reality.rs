use crate::{prelude::{*, tiles::TileTransitionConfig}, modular_assets::{TransitionType, conjoin_styles}};
use bevy::{render::camera::Camera, utils::HashMap, input::mouse::{MouseWheel, MouseScrollUnit}};
use bevy_prototype_debug_lines::DebugLines;
use uuid::Uuid;
use crate::shared::{listing::GameListing, player::Inventory};
use super::{chat::ChatMessage, Chat, Animator};

#[derive(Resource)]
pub struct Reality {
    /// Player's current position
    player_position: Transform,
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
    players_to_spawn: Vec<(User, Transform)>,
    /// Players to unload
    players_to_despawn: Vec<User>,
    /// Is this server owned by the active player?
    owns_server: bool,
    /// The state of the pause menu
    pause_menu: MenuState,
    /// A list of players that have location changes and their new locations
    players_to_move: HashMap<User, Transform>,
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
            player_position: Transform::from_xyz(0.0, 0.0, 0.0),
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
    pub fn queue_player_move(&mut self, p: User, l: Transform) {
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
    /// Sets the player's position.
    pub fn set_player_position(&mut self, position: Transform) {
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
    pub fn add_online_players(&mut self, players: Vec<(User, Transform)>) {
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
    /// Clears pending action if the held item has no action.
    pub fn system_action_none(
        mut selfs: ResMut<Reality>
    ) {
        if selfs.waiting_for_action {
            let slotted = selfs.player.inventory.hotbar[selfs.player.inventory.selected_slot];
            if let Some(item) = slotted {
                if item.action() == ItemAction::None {
                    selfs.waiting_for_action = false;
                }
            }
            else {
                selfs.waiting_for_action = false;
            }
        }
    }
    pub fn system_action_chop(
        mut commands: Commands,
        mut selfs: ResMut<Reality>,
        mut animator: ResMut<Animator>,
        mut netty: ResMut<Netty>,
        disk: Res<Disk>,
        mut objects: Query<(Entity, &mut Object)>
    ) {
        if selfs.waiting_for_action {
            let slotted = selfs.player.inventory.hotbar[selfs.player.inventory.selected_slot];
            if let Some(item) = slotted {
                let action = item.action();
                if let ItemAction::Chop(power) = action {
                    // chop time!
                    info!("Executing player action 'Chop' with power {power}");
                    // mark action for animation
                    animator.mark_action(disk.user().unwrap(), action);
                    // send animation to others
                    netty.n.send(Packet::ActionAnimation(action));
                    // check for tree in range
                    objects.for_each_mut(|(e, mut obj)| {
                        if let ObjectType::Tree(strength) = obj.rep {
                            if distance(obj.pos, selfs.player_position) < TREE_CHOP_DISTANCE {
                                if strength > power {
                                    // damage tree
                                    obj.rep = ObjectType::Tree(strength - power);
                                    // update entity on server
                                    netty.n.send(Packet::UpdateObject(obj.clone()));
                                }
                                else {
                                    // destroy tree
                                    // remove entity on server
                                    netty.n.send(Packet::RemoveObject(obj.uuid));
                                    // despawn entity locally
                                    commands.entity(e).despawn();
                                }
                            }
                        }
                    });
                    // cleanup state
                    selfs.waiting_for_action = false;
                }
            }
        }
    }
    /// Marks chunks to be rendered, downloaded, and unrendered. This system is essential to
    /// the world loading and collision loading
    pub fn system_mark_chunks(
        mut selfs: ResMut<Reality>,
        mut netty: ResMut<Netty>
    ) {
        if selfs.is_changed() && selfs.in_valid_world {
            // Chunk sizes in coordinate space
            const ENV_WIDTH: f32 = 1920.0;
            const ENV_HEIGHT: f32 = 1088.0;
            // Get the player's chunk
            let chunk_x = (selfs.player_position.translation.x / ENV_WIDTH).round() as isize;
            let chunk_y = (selfs.player_position.translation.y / ENV_HEIGHT).round() as isize;

            // Add chunks that should be loaded (5x5 around player) for download if they aren't
            // already avalable
            run_matrix_nxn(-2..=2, |x, y| {
                if !selfs.chunk_status.contains_key(&(x + chunk_x, y + chunk_y)) {
                    selfs.chunk_status.insert(
                        (x + chunk_x, y + chunk_y),
                        ChunkStatus {
                            rendered: false,
                            downloaded: false,
                            waiting_to_render: false,
                            stop_rendering: false,
                        }
                    );
                    netty.n.send(Packet::RequestChunk((chunk_x + x, chunk_y + y)));
                }
            });
            let copy_of_chunk_statuses = selfs.chunk_status.clone();
            for (chunk, status) in selfs.chunk_status.iter_mut() {
                // mark all chunks that aren't around the player to stop rendering
                if !get_matrix_nxn(-1..=1).contains(&(chunk.0 - chunk_x, chunk.1 - chunk_y)) {
                    if status.rendered {
                        status.stop_rendering = true;
                    }
                }
                else {
                    // mark all chunks that are around the player to start rendering
                    if !status.rendered && status.downloaded {
                        status.waiting_to_render = true;
                        run_matrix_nxn(-1..1, |x, y| {
                            if let Some(near_chunk_data) = copy_of_chunk_statuses.get(&(chunk.0 + x, chunk.1 + y)) {
                                if !near_chunk_data.downloaded {
                                    status.waiting_to_render = false;
                                }
                            }
                            else {
                                status.waiting_to_render = false;
                            }
                        });
                    }
                }
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
        types_serve: Res<Assets<TileTypeConfig>>,
        master_transition_serve: Res<Assets<TileTransitionMasterConfig>>,
        transition_serve: Res<Assets<TileTransitionConfig>>,
        mut atlas_serve: ResMut<Assets<TextureAtlas>>
    ) {
        let representations = &types_serve.get(&core.tiles).unwrap().states;
        let handle_transitions = &master_transition_serve.get(&core.transitions).unwrap().transitions;
        let chunk_data_copy = selfs.chunk_data.clone();
        for (chunk_location, chunk_status) in selfs.chunk_status.iter_mut() {
            if chunk_status.waiting_to_render {
                chunk_status.rendered = true;
                chunk_status.waiting_to_render = false;
                for i in 0..CHUNK_SIZE {
                    let tile_x = i % CHUNK_WIDTH;
                    let tile_y = i / CHUNK_WIDTH;
                    let layout = get_9fold_layout(
                        tile_x, tile_y,
                        &chunk_data_copy,
                        chunk_location
                    );
                    if layout.is_none() {
                        warn!("Chunks missing!");
                        chunk_status.rendered = false;
                        chunk_status.waiting_to_render = true;
                        break;
                    }
                    let layout = layout.unwrap();
                    let mut unique_tiles = vec![];
                    for tile in layout {
                        if !unique_tiles.contains(&representations[tile].name) {
                            unique_tiles.push(representations[tile].name.clone());
                        }
                    }
                    let mut main;
                    let mut sub = String::new();
                    let mut tt;
                    if unique_tiles.len() > 2 {
                        error!("Invalid terrain map! (>2 TPTSF)");
                        error!("Chunk ({}, {}), Tile ({}, {})", chunk_location.0, chunk_location.1, tile_x, tile_y);
                        continue;
                        //panic!();
                    }
                    else if unique_tiles.len() == 1 {
                        main = unique_tiles[0].clone();
                        sub = unique_tiles[0].clone();
                        tt = TransitionType::Nothing;
                    }
                    else {
                        main = representations[layout[4]].name.clone();
                        for a in unique_tiles {
                            if a != main {
                                sub = a;
                            }
                        }
                        let mut mainarr = vec![];
                        for tile in layout {
                            if representations[tile].name == main {
                                mainarr.push(true);
                            }
                            else {
                                mainarr.push(false);
                            }
                        }
                        if handle_transitions.get(&[main.clone(), sub.clone()]).is_none() {
                            // Flip everything!
                            let mut mainarr2 = vec![];
                            for elem in mainarr {
                                mainarr2.push(!elem);
                            }
                            mainarr = mainarr2;
                            let stor = main;
                            main = sub;
                            sub = stor;
                        }
                        tt = TransitionType::get_from_environment(mainarr);
                    }
                    if tt == TransitionType::Nothing {
                        sub = main.clone();
                    }
                    let handle = handle_transitions.get(&[main.clone(), sub.clone()]);
                    if handle.is_none() {
                        error!("Transition for {}, {} does not exist! (chunk {:?}, tile ({}, {}))", main, sub, chunk_location, tile_x, tile_y);
                    }
                    let handle = handle.unwrap();
                    let transition = transition_serve.get(&handle).unwrap();
                    let mut appropriate_variants = vec![];
                    for variant in &transition.variants {
                        let m_variants = conjoin_styles(variant.clone());
                        for transition in m_variants {
                            if transition.0 == tt {
                                appropriate_variants.push(transition.1);
                            }
                        }
                    }
                    if appropriate_variants.is_empty() {
                        warn!("No appropriate variants for {:?}... (tiles {} and {})", tt, main, sub);
                        tt = TransitionType::Nothing;
                        for variant in &transition.variants {
                            let m_variants = conjoin_styles(variant.clone());
                            for transition in m_variants {
                                if transition.0 == tt {
                                    appropriate_variants.push(transition.1);
                                }
                            }
                        }
                    }
                    let selected_options_option = safe_rand_from_array(appropriate_variants);
                    if let Some(selected_options) = selected_options_option {
                        if selected_options.len() == 1 {
                            commands.spawn((
                                SpriteBundle {
                                    transform: Transform::from_xyz(
                                        (-1920.0 / 2.0) + (tile_x as f32 * 64.0) + 32.0 + (1920.0 * chunk_location.0 as f32),
                                        (-1080.0 / 2.0) + (tile_y as f32 * 64.0) - 32.0 + (1088.0 * chunk_location.1 as f32),
                                        BACKGROUND
                                    ),
                                    texture: transition.images[selected_options[0]].force_sprite(),
                                    ..default()
                                },
                                Tile {
                                    chunk: *chunk_location,
                                    position: (tile_x, tile_y),
                                    transition_type: tt
                                }
                            ));
                        }
                        else if selected_options.len() == 2 {
                            let (img, width, height) = transition.images[selected_options[0]].force_sprite_sheet();
                            let sprite = TextureAtlasSprite {
                                index: selected_options[1],
                                ..default()
                            };
                            let new_atlas = TextureAtlas::from_grid(
                                img,
                                Vec2::new(64.0, 64.0),
                                width, height,
                                None, None
                            );
                            let atlas_handle = atlas_serve.add(new_atlas);
                            commands.spawn((
                                SpriteSheetBundle {
                                    transform: Transform::from_xyz(
                                        (-1920.0 / 2.0) + (tile_x as f32 * 64.0) + 32.0 + (1920.0 * chunk_location.0 as f32),
                                        (-1080.0 / 2.0) + (tile_y as f32 * 64.0) - 32.0 + (1088.0 * chunk_location.1 as f32),
                                        BACKGROUND
                                    ),
                                    sprite,
                                    texture_atlas: atlas_handle,
                                    ..default()
                                },
                                Tile {
                                    chunk: *chunk_location,
                                    position: (tile_x, tile_y),
                                    transition_type: tt
                                }
                            ));
                        }
                        else {
                            todo!()
                        }
                    }
                    else {
                        error!("No tiles selected! Unable to render...");
                    }
                }
            }
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
                    transform.translation.x = object.pos.translation.x;
                    transform.translation.y = object.pos.translation.y;
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
                ObjectType::Tree(_str) => {
                    commands.spawn((
                        SpriteBundle {
                            texture: obj_assets.tree.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    object.pos.translation.x,
                                    object.pos.translation.y,
                                    FRONT_OBJECTS
                                ),
                                rotation: Quat::default(),
                                scale: Vec3::new(0.1, 0.1, 1.0)
                            },
                            ..default()
                        },
                        object.clone()
                    ));
                }
                ObjectType::GroundItem(item) => {
                    commands.spawn((
                        SpriteBundle {
                            texture: item_assets.pick_from_item(Some(*item)),
                            transform: Transform::from_xyz(object.pos.translation.x, object.pos.translation.y, FRONT_OBJECTS),
                            ..default()
                        },
                        object.clone()
                    ));
                }
                ObjectType::Npc(_who) => {
                    commands.spawn((
                        SpriteBundle {
                            texture: npc_assets.not_animated.clone(),
                            transform: Transform::from_xyz(object.pos.translation.x, object.pos.translation.y, PLAYER_CHARACTERS),
                            ..default()
                        },
                        object.clone()
                    ));
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
            commands.spawn((
                SpriteBundle {
                    texture: textures.slot.clone(),
                    transform: Transform::from_xyz(i as f32 * 64.0 - (64.0 * 5.0), -(1080.0 / 2.0) + 32.0, UI_IMG),
                    ..Default::default()
                },
                HotbarMarker { location: i, type_: 1 },
                UILocked {}
            ));
            commands.spawn((
                SpriteBundle {
                    texture: items.none.clone(),
                    transform: Transform::from_xyz(i as f32 * 64.0 - (64.0 * 5.0), -(1080.0 / 2.0) + 32.0, UI_IMG + 0.01),
                    ..Default::default()
                },
                HotbarMarker { location: i, type_: 3 },
                UILocked {}
            ));
        }
        commands.spawn((
            SpriteBundle {
                texture: textures.selected.clone(),
                transform: Transform::from_xyz(
                        selfs.player.inventory.selected_slot as f32 * 64.0 - (64.0 * 5.0),
                        -(1080.0 / 2.0) + 32.0,
                        UI_IMG + 0.02
                    ),
                    ..Default::default()
            },
            HotbarMarker { location: selfs.player.inventory.selected_slot, type_: 2 },
            UILocked {}
        ));
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
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(location.translation.x, location.translation.y, PLAYER_CHARACTERS),
                        texture: assets.not_animated.clone(),
                        ..Default::default()
                    },
                    user
                ));
            }
        }
        selfs.players_to_spawn.clear();
    }
    pub fn system_player_unloader(
        mut selfs: ResMut<Reality>,
        mut unloads: Query<(Entity, &mut User)>,
        mut commands: Commands
    ) {
        unloads.for_each_mut(|(e, m)| {
            if selfs.players_to_despawn.contains(&m) {
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
                ((selfs.player_position.translation.x + (1920.0 / 2.0)) / 1920.0).floor() as isize,
                ((selfs.player_position.translation.y + (1088.0 / 2.0)) / 1088.0).floor() as isize
            );
            let centered_tile = (
                ((selfs.player_position.translation.x - (1920 * centered_chunk.0) as f32 + (1920.0 / 2.0)) / 64.0) as isize,
                ((selfs.player_position.translation.y - (1088 * centered_chunk.1) as f32 + (1088.0 / 2.0)) / 64.0) as isize + 1
            );
            // get a 3x3 matrix
            let mut needed_tiles: Vec<(isize, isize)> = get_matrix_nxn(-1..=1);
            // offset by centered_tile's location
            run_matrix_nxn((-1 as isize)..=1, |x, y| {
                needed_tiles[(x + 1) as usize + ((y + 1) as usize * 3)].0 += centered_tile.0;
                needed_tiles[(x + 1) as usize + ((y + 1) as usize * 3)].1 += centered_tile.1;
            });
            let mut needed_pairs = vec![];
            let mut needed_chunks = vec![];
            let l_width = CHUNK_WIDTH as isize;
            let l_height = CHUNK_HEIGHT as isize;
            for tile in needed_tiles {
                // location is right one chunk
                if tile.0 >= l_width {
                    if tile.1 < l_height && tile.1 >= 0 {
                        // location is right one chunk
                        needed_pairs.push(((centered_chunk.0 + 1, centered_chunk.1), (0, tile.1)));
                        if !needed_chunks.contains(&(centered_chunk.0 + 1, centered_chunk.1)) {
                            needed_chunks.push((centered_chunk.0 + 1, centered_chunk.1));
                        }
                    }
                    else if tile.1 >= l_height {
                        // location is right and up one chunk
                        needed_pairs.push(((centered_chunk.0 + 1, centered_chunk.1 + 1), (0, 0)));
                        if !needed_chunks.contains(&(centered_chunk.0 + 1, centered_chunk.1 + 1)) {
                            needed_chunks.push((centered_chunk.0 + 1, centered_chunk.1 + 1));
                        }
                    }
                    else {
                        // location is right and down one chunk
                        needed_pairs.push(((centered_chunk.0 + 1, centered_chunk.1 - 1), (0, l_height - 1)));
                        if !needed_chunks.contains(&(centered_chunk.0 + 1, centered_chunk.1 - 1)) {
                            needed_chunks.push((centered_chunk.0 + 1, centered_chunk.1 - 1));
                        }
                    }
                }
                else if tile.0 < 0 {
                    if tile.1 < l_height && tile.1 >= 0 {
                        // location is left one chunk
                        needed_pairs.push(((centered_chunk.0 - 1, centered_chunk.1), (l_width - 1, tile.1)));
                        if !needed_chunks.contains(&(centered_chunk.0 - 1, centered_chunk.1)) {
                            needed_chunks.push((centered_chunk.0 - 1, centered_chunk.1));
                        }
                    }
                    else if tile.1 >= l_height {
                        // location is left and up one chunk
                        needed_pairs.push(((centered_chunk.0 - 1, centered_chunk.1 + 1), (l_width - 1, 0)));
                        if !needed_chunks.contains(&(centered_chunk.0 - 1, centered_chunk.1 + 1)) {
                            needed_chunks.push((centered_chunk.0 - 1, centered_chunk.1 + 1));
                        }
                    }
                    else {
                        // location is left and down one chunk
                        needed_pairs.push(((centered_chunk.0 - 1, centered_chunk.1 - 1), (l_width - 1, l_height - 1)));
                        if !needed_chunks.contains(&(centered_chunk.0 - 1, centered_chunk.1 - 1)) {
                            needed_chunks.push((centered_chunk.0 - 1, centered_chunk.1 - 1));
                        }
                    }
                }
                else if tile.1 < l_height && tile.1 >= 0 {
                    // location is centered
                    needed_pairs.push((centered_chunk, tile));
                    if !needed_chunks.contains(&centered_chunk) {
                        needed_chunks.push(centered_chunk);
                    }
                }
                else if tile.1 >= l_height {
                    // location is up one chunk
                    needed_pairs.push(((centered_chunk.0, centered_chunk.1 + 1), (tile.0, 0)));
                    if !needed_chunks.contains(&(centered_chunk.0, centered_chunk.1 + 1)) {
                        needed_chunks.push((centered_chunk.0, centered_chunk.1 + 1));
                    }
                }
                else {
                    // location is down one chunk
                    needed_pairs.push(((centered_chunk.0, centered_chunk.1 - 1), (tile.0, l_height - 1)));
                    if !needed_chunks.contains(&(centered_chunk.0, centered_chunk.1 - 1)) {
                        needed_chunks.push((centered_chunk.0, centered_chunk.1 - 1));
                    }
                }
            }
            let mut pulled_tiles = vec![];
            queries.p0().for_each(|tile| {
                if needed_chunks.contains(&tile.chunk) {
                    for (chunk, n_tile) in &needed_pairs {
                        if tile.chunk == *chunk && tile.position == (n_tile.0 as usize, n_tile.1 as usize) {
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
                new_pos.translation.y += 4.0;
                if  !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.translation.x, new_pos.translation.y)) &&
                    !calc_player_against_objects(objects.as_slice(), (new_pos.translation.x, new_pos.translation.y)) {
                    had_movement = true;
                }
                else {
                    new_pos.translation.y -= 4.0;
                }
            }
            if keyboard.pressed(ctrls.move_down) {
                new_pos.translation.y -= 4.0;
                if  !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.translation.x, new_pos.translation.y)) &&
                    !calc_player_against_objects(objects.as_slice(), (new_pos.translation.x, new_pos.translation.y)){
                    had_movement = true;
                }
                else {
                    new_pos.translation.y += 4.0;
                }
            }
            if keyboard.pressed(ctrls.move_left) {
                new_pos.translation.x -= 4.0;
                if  !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.translation.x, new_pos.translation.y)) &&
                    !calc_player_against_objects(objects.as_slice(), (new_pos.translation.x, new_pos.translation.y)) {
                    had_movement = true;
                }
                else {
                    new_pos.translation.x += 4.0;
                }
            }
            if keyboard.pressed(ctrls.move_right) {
                new_pos.translation.x += 4.0;
                if  !calc_player_against_tiles(pulled_tiles.as_slice(), (new_pos.translation.x, new_pos.translation.y)) &&
                    !calc_player_against_objects(objects.as_slice(), (new_pos.translation.x, new_pos.translation.y)) {
                    had_movement = true;
                }
                else {
                    new_pos.translation.x -= 4.0;
                }
            }
            
            // send to server
            if had_movement {
                selfs.set_player_position(new_pos);
                netty.n.send(Packet::RequestMove(selfs.player_position));
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
                commands.spawn((
                    Text2dBundle {
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
                    },
                    PauseMenuMarker { type_: 1 },
                    UILocked {}
                ));
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
                netty.n.send(Packet::WhitelistUser(User {
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
            campos.translation.x = selfs.player_position.translation.x as f32;
            campos.translation.y = selfs.player_position.translation.y as f32;
            if selfs.chunk_status.is_empty() {
                campos.translation.x = 0.0;
                campos.translation.y = 0.0;
            }
        });
        queries.p1().for_each_mut(|mut transform| {
            transform.translation.x += selfs.player_position.translation.x as f32;
            transform.translation.y += selfs.player_position.translation.y as f32;
        });
    }
    pub fn system_player_debug_lines(
        selfs: Res<Reality>,
        mut lines: ResMut<DebugLines>
    ) {
        if PLAYER_DEBUG {
            lines.line_colored(
                Vec3::new(
                    (selfs.player_position.translation.x - (PLAYER_HITBOX.0 / 2.0)) as f32,
                    (selfs.player_position.translation.y - (PLAYER_HITBOX.1 / 2.0)) as f32,
                    DEBUG
                ),
                Vec3::new(
                    (selfs.player_position.translation.x + (PLAYER_HITBOX.0 / 2.0)) as f32,
                    (selfs.player_position.translation.y + (PLAYER_HITBOX.1 / 2.0)) as f32,
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
                    let true_x = collider.0 + (tile.position.0 as f32 * 64.0) + (tile.chunk.0 as f32 * 1920.0) - (1920.0 / 2.0);
                    let true_y = collider.1 + (tile.position.1 as f32 * 64.0) + (tile.chunk.1 as f32 * 1088.0) - (1088.0 / 2.0) - 66.0;
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
        disk: Res<Disk>,
        mut player: Query<(&mut Transform, &User)>
    ) {
        player.for_each_mut(|(mut l, m)| {
            if m == &disk.user().unwrap() {
                l.translation.x = selfs.player_position.translation.x as f32;
                l.translation.y = selfs.player_position.translation.y as f32;
            }
            if selfs.players_to_move.contains_key(&m) {
                let which = selfs.players_to_move.get(&m).unwrap();
                l.translation.x = which.translation.x as f32;
                l.translation.y = which.translation.y as f32;
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
                commands.spawn((
                    Text2dBundle {
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
                    },
                    RemoveOnStateChange {}
                ));
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
fn calc_player_against_tiles(tiles: &[Tile], player: (f32, f32)) -> bool {
    for tile in tiles {
        let offset_x = (-1920.0 / 2.0) + (tile.chunk.0 as f32 * 1920.0) + ((tile.position.0 as f32) * 64.0);
        let offset_y = (-1088.0 / 2.0) + (tile.chunk.1 as f32 * 1088.0) + ((tile.position.1 as f32 - 1.0) * 64.0);
        if tile.transition_type.collides(player, offset_x, offset_y) {
            return true;
        }
    }
    false
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ChunkStatus {
    rendered: bool,
    downloaded: bool,
    waiting_to_render: bool,
    stop_rendering: bool,
}

fn calc_player_against_objects(objects: &[Object], player: (f32, f32)) -> bool {
    for object in objects {
        if let Some(obj_size) = object.rep.collider() {
            let obj_left = object.pos.translation.x - (obj_size.0 / 2.0);
            let obj_right = obj_left + obj_size.0;
            let obj_bottom = object.pos.translation.y - (obj_size.1 / 2.0);
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

fn get_9fold_layout(
    tile_x: usize,
    tile_y: usize,
    all_chunks: &HashMap<(isize, isize), Vec<usize>>,
    chunk: &(isize, isize),
) -> Option<[usize; 9]> {
    let chunk_up_left = all_chunks.get(&(chunk.0 - 1, chunk.1 + 1))?;
    let chunk_up = all_chunks.get(&(chunk.0, chunk.1 + 1))?;
    let chunk_up_right = all_chunks.get(&(chunk.0 + 1, chunk.1 + 1))?;
    let chunk_left = all_chunks.get(&(chunk.0 - 1, chunk.1))?;
    let this_chunk = all_chunks.get(&(chunk.0, chunk.1))?;
    let chunk_right = all_chunks.get(&(chunk.0 + 1, chunk.1))?;
    let chunk_down_left = all_chunks.get(&(chunk.0 - 1, chunk.1 - 1))?;
    let chunk_down = all_chunks.get(&(chunk.0, chunk.1 - 1))?;
    let chunk_down_right = all_chunks.get(&(chunk.0 + 1, chunk.1 - 1))?;
    if tile_x > 0 {
        if tile_x < CHUNK_WIDTH - 1 {
            if tile_y > 0 {
                if tile_y < CHUNK_HEIGHT - 1 {
                    // all tiles are within this chunk
                    return Some([
                        this_chunk[crdcv(tile_x - 1, tile_y + 1)],
                        this_chunk[crdcv(tile_x, tile_y + 1)],
                        this_chunk[crdcv(tile_x + 1, tile_y + 1)],

                        this_chunk[crdcv(tile_x - 1, tile_y)],
                        this_chunk[crdcv(tile_x, tile_y)],
                        this_chunk[crdcv(tile_x + 1, tile_y)],

                        this_chunk[crdcv(tile_x - 1, tile_y - 1)],
                        this_chunk[crdcv(tile_x, tile_y - 1)],
                        this_chunk[crdcv(tile_x + 1, tile_y - 1)]
                    ]);
                }
                else {
                    // some tiles are up
                    return Some([
                        chunk_up[crdcv(tile_x - 1, 0)],
                        chunk_up[crdcv(tile_x, 0)],
                        chunk_up[crdcv(tile_x + 1, 0)],

                        this_chunk[crdcv(tile_x - 1, tile_y)],
                        this_chunk[crdcv(tile_x, tile_y)],
                        this_chunk[crdcv(tile_x + 1, tile_y)],

                        this_chunk[crdcv(tile_x - 1, tile_y - 1)],
                        this_chunk[crdcv(tile_x, tile_y - 1)],
                        this_chunk[crdcv(tile_x + 1, tile_y - 1)]
                    ]);
                }
            }
            else {
                // some tiles are down
                return Some([
                    this_chunk[crdcv(tile_x - 1, tile_y + 1)],
                    this_chunk[crdcv(tile_x, tile_y + 1)],
                    this_chunk[crdcv(tile_x + 1, tile_y + 1)],

                    this_chunk[crdcv(tile_x - 1, tile_y)],
                    this_chunk[crdcv(tile_x, tile_y)],
                    this_chunk[crdcv(tile_x + 1, tile_y)],

                    chunk_down[crdcv(tile_x - 1, CHUNK_HEIGHT - 1)],
                    chunk_down[crdcv(tile_x, CHUNK_HEIGHT - 1)],
                    chunk_down[crdcv(tile_x + 1, CHUNK_HEIGHT - 1)]
                ]);
            }
        }
        else {
            if tile_y > 0 {
                if tile_y < CHUNK_HEIGHT - 1 {
                    // some tiles are right
                    return Some([
                        this_chunk[crdcv(tile_x - 1, tile_y + 1)],
                        this_chunk[crdcv(tile_x, tile_y + 1)],
                        chunk_right[crdcv(0, tile_y + 1)],

                        this_chunk[crdcv(tile_x - 1, tile_y)],
                        this_chunk[crdcv(tile_x, tile_y)],
                        chunk_right[crdcv(0, tile_y)],

                        this_chunk[crdcv(tile_x - 1, tile_y - 1)],
                        this_chunk[crdcv(tile_x, tile_y - 1)],
                        chunk_right[crdcv(0, tile_y - 1)]
                    ]);
                }
                else {
                    // some tiles are up and right
                    return Some([
                        chunk_up[crdcv(tile_x - 1, 0)],
                        chunk_up[crdcv(tile_x, 0)],
                        chunk_up_right[crdcv(0, 0)],

                        this_chunk[crdcv(tile_x - 1, tile_y)],
                        this_chunk[crdcv(tile_x, tile_y)],
                        chunk_right[crdcv(0, tile_y)],

                        this_chunk[crdcv(tile_x - 1, tile_y - 1)],
                        this_chunk[crdcv(tile_x, tile_y - 1)],
                        chunk_right[crdcv(0, tile_y - 1)]
                    ]);
                }
            }
            else {
                // some tiles are down and right
                return Some([
                    this_chunk[crdcv(tile_x - 1, tile_y + 1)],
                    this_chunk[crdcv(tile_x, tile_y + 1)],
                    chunk_right[crdcv(0, tile_y + 1)],

                    this_chunk[crdcv(tile_x - 1, tile_y)],
                    this_chunk[crdcv(tile_x, tile_y)],
                    chunk_right[crdcv(0, tile_y)],

                    chunk_down[crdcv(tile_x - 1, CHUNK_HEIGHT - 1)],
                    chunk_down[crdcv(tile_x, CHUNK_HEIGHT - 1)],
                    chunk_down_right[crdcv(0, CHUNK_HEIGHT - 1)]
                ]);
            }
        }
    }
    else {
        if tile_y > 0 {
            if tile_y < CHUNK_HEIGHT - 1 {
                // some tiles are left
                return Some([
                    chunk_left[crdcv(CHUNK_WIDTH - 1, tile_y + 1)],
                    this_chunk[crdcv(tile_x, tile_y + 1)],
                    this_chunk[crdcv(tile_x + 1, tile_y + 1)],

                    chunk_left[crdcv(CHUNK_WIDTH - 1, tile_y)],
                    this_chunk[crdcv(tile_x, tile_y)],
                    this_chunk[crdcv(tile_x + 1, tile_y)],

                    chunk_left[crdcv(CHUNK_WIDTH - 1, tile_y - 1)],
                    this_chunk[crdcv(tile_x, tile_y - 1)],
                    this_chunk[crdcv(tile_x + 1, tile_y - 1)]
                ]);
            }
            else {
                // some tiles are up and left
                return Some([
                    chunk_up_left[crdcv(CHUNK_WIDTH - 1, 0)],
                    chunk_up[crdcv(tile_x, 0)],
                    chunk_up[crdcv(tile_x + 1, 0)],

                    chunk_left[crdcv(CHUNK_WIDTH - 1, tile_y)],
                    this_chunk[crdcv(tile_x, tile_y)],
                    this_chunk[crdcv(tile_x + 1, tile_y)],

                    chunk_left[crdcv(CHUNK_WIDTH - 1, tile_y - 1)],
                    this_chunk[crdcv(tile_x, tile_y - 1)],
                    this_chunk[crdcv(tile_x + 1, tile_y - 1)]
                ]);
            }
        }
        else {
            // some tiles are down and left
            return Some([
                chunk_left[crdcv(CHUNK_WIDTH - 1, tile_y + 1)],
                this_chunk[crdcv(tile_x, tile_y + 1)],
                this_chunk[crdcv(tile_x + 1, tile_y + 1)],

                chunk_left[crdcv(CHUNK_WIDTH - 1, tile_y)],
                this_chunk[crdcv(tile_x, tile_y)],
                this_chunk[crdcv(tile_x + 1, tile_y)],

                chunk_down_left[crdcv(CHUNK_WIDTH - 1, CHUNK_HEIGHT - 1)],
                chunk_down[crdcv(tile_x, CHUNK_HEIGHT - 1)],
                chunk_down[crdcv(tile_x + 1, CHUNK_HEIGHT - 1)]
            ]);
        }
    }
}

fn crdcv(crdx: usize, crdy: usize) -> usize {
    crdx + crdy * CHUNK_WIDTH
}
