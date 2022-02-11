use bevy::{prelude::*, app::AppExit};

use crate::{components::{CursorMarker, ldtk::{TileMarker, PlayerMarker, InGameTile}, PauseMenuMarker}, ldtk::{LDtkMap, load_level}, assets::{MapAssets, FontAssets, AnimatorAssets}, GameState, consts::{PLAYER_CHARACTERS, UI_TEXT}, shared::{netty::Packet, saves::user}};

use super::{Netty, Reality, TextBox};

pub struct UIManager {
    active_clickables: Vec<UIClickable>,
    queued_actions: Vec<UIClickAction>
}

impl UIManager {
    pub fn init() -> UIManager {
        UIManager {
            active_clickables: vec![],
            queued_actions: vec![]
        }
    }
    pub fn add_ui(&mut self, new: UIClickable) {
        println!("UI component added.");
        println!("x: {}, y: {}, w: {}, h: {}", new.location.0, new.location.1, new.size.0, new.size.1);
        self.active_clickables.push(new);
    }
    pub fn reset_ui(&mut self) {
        self.active_clickables.clear();
        self.queued_actions.clear();
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
                    if trigger == "ExitProgramQuick" {
                        true
                    }
                    else {
                        false
                    }
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
        println!("Click occurred at ({}, {})", location.0, location.1);
        let mut removed = 0;
        for (index, clickable) in self.active_clickables.clone().iter().enumerate() {
            if clickable.is_contained(location) {
                println!("Queued an action.");
                self.queued_actions.push(clickable.action.clone());
                if clickable.removed_on_use {
                    self.active_clickables.remove(index - removed);
                    removed += 1;
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct UIClickable {
    pub action: UIClickAction,
    pub location: (f32, f32),
    pub size: (f32, f32),
    pub removed_on_use: bool
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

pub fn ui_manager(
    btn: Res<Input<MouseButton>>,
    mut man: ResMut<UIManager>,
    mut qset: QuerySet<(
        QueryState<&mut Transform, With<CursorMarker>>,
        QueryState<&mut Transform, With<Camera>>
    )>,
) {
    let mut camx = 0.0;
    let mut camy = 0.0;
    for transform in qset.q1().iter_mut() {
        camx = transform.translation.x;
        camy = transform.translation.y;
    }
    if btn.just_pressed(MouseButton::Left) {
        for location in qset.q0().iter_mut() {
            man.clicked((location.translation.x - camx, location.translation.y - camy));
        }
    }
}

pub fn ui_game(
    mut commands: Commands,
    unloads: Query<Entity, With<TileMarker>>,
    target_materials: Option<Res<AnimatorAssets>>,
    mut state: ResMut<State<GameState>>,
    mut netty: ResMut<Netty>,
    mut man: ResMut<UIManager>
) {
    if let Some(materials) = target_materials {
        if let Some(game_id) = man.join_game() {
            state.replace(GameState::Play).unwrap();
            commands.spawn_bundle(SpriteBundle {
                texture: materials.placeholder.clone(),
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    PLAYER_CHARACTERS
                ),
                ..Default::default()
            }).insert(PlayerMarker { user: user().unwrap(), isme: true });
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
            transform: Transform::from_xyz(0.0, -100.0, UI_TEXT),
            ..Default::default()
        }).insert(PauseMenuMarker {});
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
                    }
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                }
            },
            ..Default::default()
        }).insert(crate::components::TextBox {});
        tb.clear_buffer();
    }
}

pub fn ui_disconnect_game(
    mut commands: Commands,
    mut man: ResMut<UIManager>,
    mut netty: ResMut<Netty>,
    mut state: ResMut<State<GameState>>,
    mut reality: ResMut<Reality>,
    mut qset: QuerySet<(
        QueryState<Entity, With<PlayerMarker>>,
        QueryState<Entity, With<TileMarker>>,
        QueryState<Entity, With<InGameTile>>,
        QueryState<Entity, With<PauseMenuMarker>>
    )>,
) {
    if man.gameplay_trigger() == Some(String::from("LeaveGame")) {
        man.next();
        man.reset_ui();
        netty.say(Packet::LeaveWorld);
        state.set(GameState::TitleScreen).unwrap();
        qset.q0().for_each_mut(|e| {
            commands.entity(e).despawn();
        });
        qset.q1().for_each_mut(|e| {
            commands.entity(e).despawn();
        });
        qset.q2().for_each_mut(|e| {
            commands.entity(e).despawn();
        });
        qset.q3().for_each_mut(|e| {
            commands.entity(e).despawn();
        });
        man.add_ui(UIClickable {
            action: UIClickAction::ChangeScene(String::from("Title_screen")),
            location: (-2.5, -2.5),
            size: (5.0, 5.0),
            removed_on_use: false
        });
        man.clicked((0.0, 0.0));
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
                            state.set(GameState::Settings).unwrap();
                        }
                        "Create_world" => {
                            state.set(GameState::MakeGame).unwrap();
                        }
                        "Create_profile" => {
                            state.set(GameState::MakeUser).unwrap();
                        }
                        "Server_list" => {
                            state.set(GameState::ServerList).unwrap();
                        }
                        "Title_screen" => {
                            state.set(GameState::TitleScreen).unwrap();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
