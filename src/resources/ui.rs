use bevy::{prelude::*, app::AppExit};
use bevy_prototype_debug_lines::DebugLines;

use crate::{components::{CursorMarker, ldtk::{TileMarker, PlayerMarker, Tile}, PauseMenuMarker, GamePosition, UILocked, HotbarMarker, SettingsPageComp}, ldtk::{LDtkMap, load_level}, assets::{MapAssets, FontAssets, AnimatorAssets}, GameState, consts::{PLAYER_CHARACTERS, UI_TEXT, UI_DEBUG, DEBUG, CURSOR_OFFSET}, shared::{netty::Packet, object::Object}};

use super::{Netty, Reality, TextBox, Disk};

pub enum SettingsPage {
    Sound,
    Video,
    Gameplay,
    Online
}

pub struct UIManager {
    active_clickables: Vec<UIClickable>,
    queued_actions: Vec<UIClickAction>,
    queue_player_action: bool,
    settings_page: SettingsPage
}

impl UIManager {
    pub fn init() -> UIManager {
        UIManager {
            active_clickables: vec![],
            queued_actions: vec![],
            queue_player_action: false,
            settings_page: SettingsPage::Video
        }
    }
    pub fn add_ui(&mut self, new: UIClickable) {
        trace!("UI component added.");
        trace!("x: {}, y: {}, w: {}, h: {}", new.location.0, new.location.1, new.size.0, new.size.1);
        self.active_clickables.push(new);
    }
    pub fn reset_ui(&mut self) {
        self.active_clickables.clear();
        self.queued_actions.clear();
    }
    pub fn remove_tag(&mut self, tag: &str) {
        let mut removed = 0;
        for (index, object) in self.active_clickables.clone().iter().enumerate() {
            if object.tag == Some(tag.to_string()) {
                self.active_clickables.remove(index - removed);
                removed += 1;
            }
        }
    }
    fn scene_changes(&mut self) -> Option<String> {
        if self.queued_actions.get(0).is_some() {
            match self.queued_actions[0].clone() {
                UIClickAction::ChangeScene(scene) => {
                    self.next();
                    Some(scene)
                }
                _ => {
                    None
                }
            }
        }
        else {
            None
        }
    }
    fn join_game(&mut self) -> Option<usize> {
        if self.queued_actions.get(0).is_some() {
            match self.queued_actions[0].clone() {
                UIClickAction::JoinWorld(world) => {
                    self.next();
                    Some(world)
                }
                _ => {
                    None
                }
            }
        }
        else {
            None
        }
    }
    fn quick_exit(&mut self) -> bool {
        if self.queued_actions.get(0).is_some() {
            match self.queued_actions[0].clone() {
                UIClickAction::GameplayTrigger(trigger) => {
                    trigger == "ExitProgramQuick"
                }
                _ => {
                    false
                }
            }
        }
        else {
            false
        }
    }
    fn gameplay_trigger(&mut self) -> Option<String> {
        if self.queued_actions.get(0).is_some() {
            match self.queued_actions[0].clone() {
                UIClickAction::GameplayTrigger(trigger) => {
                    Some(trigger)
                }
                _ => {
                    None
                }
            }
        }
        else {
            None
        }
    }
    fn next(&mut self) {
        self.queued_actions.remove(0);
    }
    fn clicked(&mut self, location: (f32, f32)) {
        let mut removed = 0;
        let mut did_action = false;
        for (index, clickable) in self.active_clickables.clone().iter().enumerate() {
            if clickable.is_contained(location) {
                did_action = true;
                self.queued_actions.push(clickable.action.clone());
                if clickable.removed_on_use {
                    self.active_clickables.remove(index - removed);
                    removed += 1;
                }
            }
        }
        if !did_action {
            self.queue_player_action = true;
        }
    }
    fn grab_player_action(&mut self) -> bool {
        if self.queue_player_action {
            self.queue_player_action = false;
            return true;
        }
        false
    }
}

#[derive(Clone)]
pub struct UIClickable {
    pub action: UIClickAction,
    pub location: (f32, f32),
    pub size: (f32, f32),
    pub removed_on_use: bool,
    pub tag: Option<String>
}

impl UIClickable {
    fn is_contained(&self, point: (f32, f32)) -> bool {
        if 
            point.0 > self.location.0 &&
            point.1 > self.location.1 - self.size.1 &&
            point.0 < self.location.0 + self.size.0 &&
            point.1 < self.location.1
        {
            return true
        }
        false
    }
}

#[derive(Clone)]
pub enum UIClickAction {
    ChangeScene(String),
    GameplayTrigger(String),
    JoinWorld(usize)
}

pub fn ui_debug_lines(
    man: Res<UIManager>,
    mut lines: ResMut<DebugLines>,
) {
    if UI_DEBUG {
        for clickable in &man.active_clickables {
            lines.line_colored(
                Vec3::new(clickable.location.0, clickable.location.1, DEBUG),
                Vec3::new(clickable.location.0 + clickable.size.0, clickable.location.1, DEBUG),
                0.0,
                Color::RED
            );
            lines.line_colored(
                Vec3::new(clickable.location.0, clickable.location.1, DEBUG),
                Vec3::new(clickable.location.0, clickable.location.1 - clickable.size.1, DEBUG),
                0.0,
                Color::RED
            );
            lines.line_colored(
                Vec3::new(clickable.location.0 + clickable.size.0, clickable.location.1, DEBUG),
                Vec3::new(clickable.location.0 + clickable.size.0, clickable.location.1 - clickable.size.1, DEBUG),
                0.0,
                Color::RED
            );
            lines.line_colored(
                Vec3::new(clickable.location.0, clickable.location.1 - clickable.size.1, DEBUG),
                Vec3::new(clickable.location.0 + clickable.size.0, clickable.location.1 - clickable.size.1, DEBUG),
                0.0,
                Color::RED
            );
        }
    }
}

pub fn ui_forward(
    mut man: ResMut<UIManager>,
    mut reality: ResMut<Reality>
) {
    if man.grab_player_action() {
        reality.queue_action();
    }
}

pub fn ui_manager(
    btn: Res<Input<MouseButton>>,
    mut man: ResMut<UIManager>,
    mut query: Query<&mut Transform, With<CursorMarker>>,
) {
    if btn.just_pressed(MouseButton::Left) {
        for location in query.iter_mut() {
            man.clicked((location.translation.x + CURSOR_OFFSET[0], location.translation.y + CURSOR_OFFSET[1]));
        }
    }
}

pub fn ui_game(
    mut commands: Commands,
    unloads: Query<Entity, With<TileMarker>>,
    target_materials: Option<Res<AnimatorAssets>>,
    mut state: ResMut<State<GameState>>,
    mut netty: ResMut<Netty>,
    mut man: ResMut<UIManager>,
    disk: Res<Disk>
) {
    if let Some(materials) = target_materials {
        if let Some(game_id) = man.join_game() {
            state.replace(GameState::Play).unwrap();
            commands.spawn_bundle(SpriteBundle {
                texture: materials.not_animated.clone(),
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    PLAYER_CHARACTERS
                ),
                ..Default::default()
            }).insert(PlayerMarker { user: disk.user().unwrap(), isme: true });
            netty.say(Packet::JoinWorld(game_id));
            unloads.for_each(|e| {
                commands.entity(e).despawn_recursive();
            });
            man.reset_ui();
        }
    }
}

pub fn ui_quick_exit(
    mut man: ResMut<UIManager>,
    mut exit: EventWriter<AppExit>
) {
    if man.quick_exit() {
        exit.send(AppExit);
    }
}

pub fn ui_close_pause_menu(
    mut man: ResMut<UIManager>,
    mut selfs: ResMut<Reality>,
) {
    if man.gameplay_trigger() == Some(String::from("ClosePauseMenu")) {
        man.next();
        man.reset_ui();
        selfs.pause_closed();
    }
}

pub fn ui_invite_menu(
    mut commands: Commands,
    mut man: ResMut<UIManager>,
    fonts: Option<Res<FontAssets>>,
    desps: Query<Entity, With<PauseMenuMarker>>,
    mut tb: ResMut<TextBox>
) {
    if man.gameplay_trigger() == Some(String::from("InvitePlayer")) {
        man.next();
        man.reset_ui();
        desps.for_each(|e| {
            commands.entity(e).despawn();
        });
        commands.spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: String::from("What player? (ex PlayerName#1234)\n"),
                        style: TextStyle {
                            font: fonts.as_ref().unwrap().simvoni.clone(),
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
            transform: Transform::from_xyz(0.0, 100.0, UI_TEXT),
            ..Default::default()
        }).insert(PauseMenuMarker { type_: 2 }).insert(UILocked {});
        commands.spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: String::new(),
                        style: TextStyle {
                            font: fonts.as_ref().unwrap().simvoni.clone(),
                            font_size: 55.0,
                            color: Color::BLACK
                        }
                    },
                    TextSection {
                        value: String::new(),
                        style: TextStyle {
                            font: fonts.as_ref().unwrap().simvoni.clone(),
                            font_size: 55.0,
                            color: Color::GRAY
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
        }).insert(crate::components::TextBox {}).insert(PauseMenuMarker { type_: 1 }).insert(UILocked {});
        tb.clear_buffer();
    }
}

pub fn ui_settings_camera(
    mut query: Query<&mut Transform, With<Camera>>
) {
    query.for_each_mut(|mut position| {
        position.translation.x = 0.0;
        position.translation.y = 0.0;
    });
}

pub fn ui_settings_page(
    mut commands: Commands,
    despawns: Query<Entity, With<SettingsPageComp>>,
    mut man: ResMut<UIManager>,
    fonts: Res<FontAssets>,
    mut disk: ResMut<Disk>
) {
    if let Some(trigger) = man.gameplay_trigger() {
        match trigger.as_str() {
            "SoundSettings" => {
                man.settings_page = SettingsPage::Sound;
                despawns.for_each(|entity| {
                    commands.entity(entity).despawn();
                });
            }
            "VideoSettings" => {
                despawns.for_each(|entity| {
                    commands.entity(entity).despawn();
                });
                man.settings_page = SettingsPage::Video;
                man.remove_tag("Settings");
                man.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("ToggleFullscreen")),
                    location: (0.0, 0.0),
                    size: (200.0, 36.0),
                    removed_on_use: false,
                    tag: Some(String::from("Settings"))
                });
                man.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("IncreaseWindowScaling")),
                    location: (0.0, 0.0), // TODO
                    size: (0.0, 0.0), // TODO
                    removed_on_use: false,
                    tag: Some(String::from("Settings"))
                });
                man.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("DecreaseWindowScaling")),
                    location: (0.0, 0.0), // TODO
                    size: (0.0, 0.0), // TODO
                    removed_on_use: false,
                    tag: Some(String::from("Settings"))
                });
                man.add_ui(UIClickable {
                    action: UIClickAction::GameplayTrigger(String::from("ToggleVSync")),
                    location: (0.0, 0.0), // TODO
                    size: (0.0, 0.0), // TODO
                    removed_on_use: false,
                    tag: Some(String::from("Settings"))
                });
                commands.spawn_bundle(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("Fullscreen: {}", disk.window_config().fullscreen),
                            style: TextStyle {
                                font: fonts.simvoni.clone(),
                                font_size: 36.0,
                                color: Color::BLACK
                            }
                        }],
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Top,
                            horizontal: HorizontalAlign::Left
                        }
                    },
                    transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
                    ..default()
                }).insert(SettingsPageComp { type_: 1 });
            }
            "GameplaySettings" => {
                man.settings_page = SettingsPage::Gameplay;
                despawns.for_each(|entity| {
                    commands.entity(entity).despawn();
                });
            }
            "OnlineSettings" => {
                man.settings_page = SettingsPage::Online;
                despawns.for_each(|entity| {
                    commands.entity(entity).despawn();
                });
            }
            "LeaveSettings" => {
                man.settings_page = SettingsPage::Video;
                despawns.for_each(|entity| {
                    commands.entity(entity).despawn();
                });
            }
            "ToggleFullscreen" => {
                let mut winconf = disk.window_config();
                winconf.fullscreen = !winconf.fullscreen;
                disk.update_window_config(winconf);
            }
            _ => { return } // do nothing
        }
        man.next();
    }
}

pub fn ui_settings_text_updater(
    mut query: Query<(&mut Text, &SettingsPageComp)>,
    disk: Res<Disk>
) {
    query.for_each_mut(|(mut text, component)| {
        if component.type_ == 1 {
            text.sections[0].value = format!("Fullscreen: {}", disk.window_config().fullscreen);
        }
    });
}

pub fn ui_close_settings(
    mut commands: Commands,
    mut man: ResMut<UIManager>,
    mut state: ResMut<State<GameState>>,
    query: Query<Entity, With<TileMarker>>
) {
    if man.gameplay_trigger() == Some(String::from("LeaveSettings")) {
        query.for_each(|e| {
            commands.entity(e).despawn();
        });
        man.next();
        man.reset_ui();
        state.pop().unwrap();
    }
}

pub fn ui_resume_game_settings(
    mut uiman: ResMut<UIManager>
) {
    uiman.add_ui(UIClickable {
        action: UIClickAction::GameplayTrigger(String::from("ClosePauseMenu")),
        location: (-150.0, 110.0 - 27.5),
        size: (300.0, 55.0),
        removed_on_use: false,
        tag: None
    });
    uiman.add_ui(UIClickable {
        action: UIClickAction::GameplayTrigger(String::from("InvitePlayer")),
        location: (-150.0, 55.0 - 27.5),
        size: (300.0, 55.0),
        removed_on_use: false,
        tag: None
    });
    uiman.add_ui(UIClickable {
        action: UIClickAction::ChangeScene(String::from("Settings")),
        location: (-150.0, -27.5),
        size: (300.0, 55.0),
        removed_on_use: false,
        tag: None
    });
    uiman.add_ui(UIClickable {
        action: UIClickAction::GameplayTrigger(String::from("LeaveGame")),
        location: (-150.0, -55.0 - 27.5),
        size: (300.0, 55.0),
        removed_on_use: false,
        tag: None
    });
}

pub fn ui_disconnect_game(
    mut commands: Commands,
    mut man: ResMut<UIManager>,
    mut netty: ResMut<Netty>,
    mut state: ResMut<State<GameState>>,
    mut reality: ResMut<Reality>,
    mut query: Query<
        Entity,
        Or<(
            With<PlayerMarker>,
            With<TileMarker>,
            With<Tile>,
            With<PauseMenuMarker>,
            With<HotbarMarker>,
            With<Object>
        )>
    >
) {
    if man.gameplay_trigger() == Some(String::from("LeaveGame")) {
        man.next();
        man.reset_ui();
        netty.say(Packet::LeaveWorld);
        state.set(GameState::TitleScreen).unwrap();
        query.for_each_mut(|e| {
            commands.entity(e).despawn();
        });
        man.add_ui(UIClickable {
            action: UIClickAction::ChangeScene(String::from("Title_screen")),
            location: (-2.5, -2.5),
            size: (5.0, 5.0),
            removed_on_use: true,
            tag: None
        });
        man.clicked((0.0, 0.0));
        // We do this so UI doesn't get misaligned
        reality.set_player_position(GamePosition::zero());
        // Fully reset because making things not conflict is hard :P
        reality.reset();
    }
}

pub fn ui_scene(
    mut commands: Commands,
    unloads: Query<Entity, With<TileMarker>>,
    mapsatt: Option<ResMut<Assets<LDtkMap>>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    target_mapsatt: Option<Res<MapAssets>>,
    mut state: ResMut<State<GameState>>,
    font_assetsatt: Option<Res<FontAssets>>,
    mut man: ResMut<UIManager>
) {
    if let Some(font_assets) = font_assetsatt {
        if let Some(target_maps) = target_mapsatt {
            if let Some(mut maps) = mapsatt {
                if let Some(goto) = man.scene_changes() {
                    man.reset_ui();
                    let a = maps.get_mut(target_maps.core.clone()).unwrap();
                    let level = a.get_level(goto.as_str());
                    load_level(unloads, level, a, texture_atlases, font_assets.clone(), man, &mut commands);
                    match goto.as_str() {
                        "Settings" => {
                            state.push(GameState::Settings).unwrap();
                        }
                        "Create_world" => {
                            state.replace(GameState::MakeGame).unwrap();
                        }
                        "Create_profile" => {
                            state.replace(GameState::MakeUser).unwrap();
                        }
                        "Server_list" => {
                            state.replace(GameState::ServerList).unwrap();
                        }
                        "Title_screen" => {
                            state.replace(GameState::TitleScreen).unwrap();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
