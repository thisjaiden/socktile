use bevy::prelude::*;

use crate::{components::{CursorMarker, ldtk::TileMarker}, ldtk::{LDtkMap, load_level}, MapAssets, GameState, FontAssets};

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
                    self.queued_actions.remove(0);
                    return Some(scene);
                }
                _ => {
                    return None;
                }
            }
        }
        else {
            return None;
        }
    }
    fn clicked(&mut self, location: (f32, f32)) {
        println!("Click occurred at ({}, {})", location.0, location.1);
        let approx_loc = (location.0 + (1920.0 / 2.0), location.1 + (1080.0 / 2.0));
        //println!("Approx to ({}, {})", approx_loc.0, approx_loc.1);
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
    GameplayTrigger(String)
}

pub fn ui_manager(
    btn: Res<Input<MouseButton>>,
    mut man: ResMut<UIManager>,
    mut cursors: Query<&mut Transform, With<CursorMarker>>
) {
    if btn.just_pressed(MouseButton::Left) {
        for location in cursors.iter_mut() {
            man.clicked((location.translation.x, location.translation.y));
        }
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
                    let a = maps.get_mut(target_maps.player.clone()).unwrap();
                    let level = a.get_level(goto.as_str());
                    load_level(unloads, level, a, texture_atlases, font_assets, man, &mut commands);
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
