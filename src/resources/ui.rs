use crate::prelude::*;
use bevy::app::AppExit;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_prototype_debug_lines::DebugLines;

use super::{Reality, TextBox};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SettingsPage {
    Sound,
    Video,
    Gameplay,
    Online,
}

#[derive(Resource)]
pub struct UIManager {
    active_clickables: Vec<UIClickable>,
    pub queued_action: Option<UIClickAction>,
    queue_player_action: bool,
    pub settings_page: SettingsPage,
    pub on_page: bool,
}

impl UIManager {
    pub fn init() -> UIManager {
        UIManager {
            active_clickables: vec![],
            queued_action: None,
            queue_player_action: false,
            settings_page: SettingsPage::Video,
            on_page: false,
        }
    }
    pub fn add_ui(&mut self, new: UIClickable) {
        trace!("UI component added.");
        trace!(
            "x: {}, y: {}, w: {}, h: {}",
            new.location.0,
            new.location.1,
            new.size.0,
            new.size.1
        );
        self.active_clickables.push(new);
    }
    pub fn reset_ui(&mut self) {
        self.active_clickables.clear();
        self.queued_action = None;
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
    fn join_game(&mut self) -> Option<usize> {
        if self.queued_action.is_some() {
            match self.queued_action.unwrap() {
                UIClickAction::JoinWorld(world) => Some(world),
                _ => None,
            }
        }
        else {
            None
        }
    }
    fn clicked(&mut self, location: (f32, f32)) {
        let mut removed = 0;
        let mut did_action = false;
        for (index, clickable) in self.active_clickables.clone().iter().enumerate() {
            if clickable.is_contained(location) {
                did_action = true;
                if self.queued_action.is_some() {
                    warn!("A UI action occured while one was waiting. Discarding old action");
                }
                info!("Click occured on an actionable location.");
                self.queued_action = Some(clickable.action);
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
}

#[derive(Clone)]
pub struct UIClickable {
    pub action: UIClickAction,
    pub location: (f32, f32),
    pub size: (f32, f32),
    pub removed_on_use: bool,
    pub tag: Option<String>,
}

impl UIClickable {
    fn is_contained(&self, point: (f32, f32)) -> bool {
        if 
            point.0 > self.location.0 &&
            point.1 > self.location.1 - self.size.1 &&
            point.0 < self.location.0 + self.size.0 &&
            point.1 < self.location.1
        {
            return true;
        }
        false
    }
}

impl Default for UIClickable {
    fn default() -> UIClickable {
        UIClickable {
            action: UIClickAction::CloseProgram,
            location: (0.0, 0.0),
            size: (0.0, 0.0),
            removed_on_use: true,
            tag: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UIClickAction {
    CloseProgram,
    ClosePauseMenu,
    ToggleFullscreen,
    IncreaseWindowScaling,
    DecreaseWindowScaling,
    InvitePlayer,
    DisconnectFromWorld,
    GoToCreateWorld,
    GoToTitleScreen,
    CreateWorld,
    ViewWorldList,
    OpenSettings,
    CloseSettings,
    TabSoundSettings,
    TabVideoSettings,
    TabGameplaySettings,
    TabOnlineSettings,
    JoinWorld(usize),
}

pub fn ui_debug_lines(man: Res<UIManager>, mut lines: ResMut<DebugLines>) {
    if UI_DEBUG {
        for clickable in &man.active_clickables {
            lines.line_colored(
                Vec3::new(clickable.location.0, clickable.location.1, DEBUG),
                Vec3::new(
                    clickable.location.0 + clickable.size.0,
                    clickable.location.1,
                    DEBUG,
                ),
                0.0,
                Color::RED,
            );
            lines.line_colored(
                Vec3::new(clickable.location.0, clickable.location.1, DEBUG),
                Vec3::new(
                    clickable.location.0,
                    clickable.location.1 - clickable.size.1,
                    DEBUG,
                ),
                0.0,
                Color::RED,
            );
            lines.line_colored(
                Vec3::new(
                    clickable.location.0 + clickable.size.0,
                    clickable.location.1,
                    DEBUG,
                ),
                Vec3::new(
                    clickable.location.0 + clickable.size.0,
                    clickable.location.1 - clickable.size.1,
                    DEBUG,
                ),
                0.0,
                Color::RED,
            );
            lines.line_colored(
                Vec3::new(
                    clickable.location.0,
                    clickable.location.1 - clickable.size.1,
                    DEBUG,
                ),
                Vec3::new(
                    clickable.location.0 + clickable.size.0,
                    clickable.location.1 - clickable.size.1,
                    DEBUG,
                ),
                0.0,
                Color::RED,
            );
        }
    }
}

pub fn ui_forward(mut man: ResMut<UIManager>, mut reality: ResMut<Reality>) {
    if man.queue_player_action {
        man.queue_player_action = false;
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
            man.clicked((
                location.translation.x + CURSOR_OFFSET[0],
                location.translation.y + CURSOR_OFFSET[1],
            ));
        }
    }
}

pub fn ui_game(
    mut commands: Commands,
    target_materials: Option<Res<AnimatorAssets>>,
    mut state: ResMut<State<GameState>>,
    mut netty: ResMut<Netty>,
    mut man: ResMut<UIManager>,
    disk: Res<Disk>,
    audio: Res<Audio>,
    core: Res<CoreAssets>,
    audio_serve: Res<Assets<AudioSamples>>,
) {
    if let Some(materials) = target_materials {
        if let Some(game_id) = man.join_game() {
            let samples = audio_serve.get(&core.audio).unwrap();
            audio.play(samples.get("click"));
            state.replace(GameState::Play).unwrap();
            commands.spawn((
                SpriteBundle {
                    texture: materials.not_animated.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, PLAYER_CHARACTERS),
                    ..Default::default()
                },
                disk.user().unwrap(),
            ));
            netty.send(Packet::JoinWorld(game_id));
            man.reset_ui();
        }
    }
}

pub fn ui_quick_exit(man: Res<UIManager>, mut exit: EventWriter<AppExit>) {
    if man.queued_action == Some(UIClickAction::CloseProgram) {
        exit.send(AppExit);
    }
}

pub fn ui_close_pause_menu(mut man: ResMut<UIManager>, mut selfs: ResMut<Reality>) {
    if man.queued_action == Some(UIClickAction::ClosePauseMenu) {
        man.reset_ui();
        selfs.pause_closed();
    }
}

pub fn ui_invite_menu(
    mut commands: Commands,
    mut man: ResMut<UIManager>,
    fonts: Option<Res<FontAssets>>,
    desps: Query<Entity, With<PauseMenuMarker>>,
    mut tb: ResMut<TextBox>,
) {
    if man.queued_action == Some(UIClickAction::InvitePlayer) {
        man.reset_ui();
        desps.for_each(|e| {
            commands.entity(e).despawn();
        });
        commands.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: String::from("What player? (ex PlayerName#1234)\n"),
                        style: TextStyle {
                            font: fonts.as_ref().unwrap().simvoni.clone(),
                            font_size: 55.0,
                            color: Color::BLACK,
                        },
                    }],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                transform: Transform::from_xyz(0.0, 100.0, UI_TEXT),
                ..Default::default()
            },
            PauseMenuMarker { type_: 2 },
            UILocked {},
        ));
        commands.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: String::new(),
                            style: TextStyle {
                                font: fonts.as_ref().unwrap().simvoni.clone(),
                                font_size: 55.0,
                                color: Color::BLACK,
                            },
                        },
                        TextSection {
                            value: String::new(),
                            style: TextStyle {
                                font: fonts.as_ref().unwrap().simvoni.clone(),
                                font_size: 55.0,
                                color: Color::GRAY,
                            },
                        },
                    ],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
                ..Default::default()
            },
            crate::components::TextBox {},
            PauseMenuMarker { type_: 1 },
            UILocked {},
        ));
        tb.clear_buffer();
    }
}

pub fn ui_settings_camera(mut query: Query<&mut Transform, With<Camera>>) {
    query.for_each_mut(|mut position| {
        position.translation.x = 0.0;
        position.translation.y = 0.0;
    });
}

pub fn ui_settings_tab(mut man: ResMut<UIManager>) {
    if man.queued_action == Some(UIClickAction::TabVideoSettings) {
        man.on_page = false;
        man.settings_page = SettingsPage::Video;
        man.remove_tag("Settings");
    }
    else if man.queued_action == Some(UIClickAction::TabSoundSettings) {
        man.on_page = false;
        man.settings_page = SettingsPage::Sound;
        man.remove_tag("Settings");
    }
    else if man.queued_action == Some(UIClickAction::TabGameplaySettings) {
        man.on_page = false;
        man.settings_page = SettingsPage::Gameplay;
        man.remove_tag("Settings");
    }
    else if man.queued_action == Some(UIClickAction::TabOnlineSettings) {
        man.on_page = false;
        man.settings_page = SettingsPage::Online;
        man.remove_tag("Settings");
    }
}

pub fn ui_toggle_fullscreen(mut man: ResMut<UIManager>, mut disk: ResMut<Disk>) {
    if man.queued_action == Some(UIClickAction::ToggleFullscreen) {
        let mut winconf = disk.window_config();
        winconf.fullscreen = !winconf.fullscreen;
        disk.update_window_config(winconf);
        man.queued_action = None;
    }
}

pub fn ui_increase_scaling(mut man: ResMut<UIManager>, mut disk: ResMut<Disk>) {
    if man.queued_action == Some(UIClickAction::IncreaseWindowScaling) {
        let mut winconf = disk.window_config();
        winconf.scale_factor += 0.25;
        disk.update_window_config(winconf);
        man.queued_action = None;
    }
}

pub fn ui_decrease_scaling(mut man: ResMut<UIManager>, mut disk: ResMut<Disk>) {
    if man.queued_action == Some(UIClickAction::DecreaseWindowScaling) {
        let mut winconf = disk.window_config();
        winconf.scale_factor -= 0.25;
        disk.update_window_config(winconf);
        man.queued_action = None;
    }
}

pub fn ui_settings_text_updater(
    mut query: Query<(&mut Text, &SettingsPageComp)>,
    disk: Res<Disk>,
    core: Res<CoreAssets>,
    lang_serve: Res<Assets<LanguageKeys>>,
) {
    let lang = lang_serve.get(&core.lang).unwrap();
    query.for_each_mut(|(mut text, component)| {
        if component.type_ == 1 {
            let txtout;
            if disk.window_config().fullscreen {
                txtout = lang.get("en_us.core.settings.fullscreen.on");
            }
            else {
                txtout = lang.get("en_us.core.settings.fullscreen.off");
            }
            text.sections[0].value = txtout;
        }
        else if component.type_ == 4 {
            text.sections[0].value = format!("{}", disk.window_config().scale_factor);
        }
    });
}

pub fn ui_close_settings(
    mut commands: Commands,
    mut man: ResMut<UIManager>,
    mut state: ResMut<State<GameState>>,
    query: Query<Entity, With<RemoveOnStateChange>>,
) {
    if man.queued_action == Some(UIClickAction::CloseSettings) {
        query.for_each(|e| {
            commands.entity(e).despawn();
        });
        man.reset_ui();
        state.pop().unwrap();
    }
}

pub fn ui_resume_game_settings(mut uiman: ResMut<UIManager>) {
    uiman.add_ui(UIClickable {
        action: UIClickAction::ClosePauseMenu,
        location: (-150.0, 110.0 - 27.5),
        size: (300.0, 55.0),
        removed_on_use: false,
        tag: None,
    });
    uiman.add_ui(UIClickable {
        action: UIClickAction::InvitePlayer,
        location: (-150.0, 55.0 - 27.5),
        size: (300.0, 55.0),
        removed_on_use: false,
        tag: None,
    });
    uiman.add_ui(UIClickable {
        action: UIClickAction::OpenSettings,
        location: (-150.0, -27.5),
        size: (300.0, 55.0),
        removed_on_use: false,
        tag: None,
    });
    uiman.add_ui(UIClickable {
        action: UIClickAction::DisconnectFromWorld,
        location: (-150.0, -55.0 - 27.5),
        size: (300.0, 55.0),
        removed_on_use: false,
        tag: None,
    });
}

pub fn ui_disconnect_game(
    mut commands: Commands,
    mut man: ResMut<UIManager>,
    mut netty: ResMut<Netty>,
    mut state: ResMut<State<GameState>>,
    mut reality: ResMut<Reality>,
    audio: Res<Audio>,
    core: Res<CoreAssets>,
    audio_serve: Res<Assets<AudioSamples>>,
    mut query: Query<
        Entity,
        Or<(
            With<User>,
            With<Tile>,
            With<PauseMenuMarker>,
            With<HotbarMarker>,
            With<Object>,
            With<AnimatedSprite>,
        )>,
    >,
) {
    if man.queued_action == Some(UIClickAction::DisconnectFromWorld) {
        let samples = audio_serve.get(&core.audio).unwrap();
        audio.play(samples.get("click"));
        man.reset_ui();
        netty.send(Packet::LeaveWorld);
        query.for_each_mut(|e| {
            commands.entity(e).despawn();
        });
        // We do this so UI doesn't get misaligned
        reality.set_player_position(Transform::from_xyz(0.0, 0.0, 0.0));
        // Fully reset because making things not conflict is hard :P
        reality.reset();
        // Switch to titlescreen
        state.overwrite_set(GameState::TitleScreen).unwrap();
    }
}

pub fn ui_return_create_world(
    mut state: ResMut<State<GameState>>,
    mut man: ResMut<UIManager>,
    audio: Res<Audio>,
    core: Res<CoreAssets>,
    audio_serve: Res<Assets<AudioSamples>>,
) {
    if man.queued_action == Some(UIClickAction::GoToCreateWorld) {
        let samples = audio_serve.get(&core.audio).unwrap();
        audio.play(samples.get("click"));
        man.reset_ui();
        state.replace(GameState::MakeGame).unwrap();
    }
}

pub fn ui_view_worlds(
    mut state: ResMut<State<GameState>>,
    mut man: ResMut<UIManager>,
    audio: Res<Audio>,
    core: Res<CoreAssets>,
    audio_serve: Res<Assets<AudioSamples>>,
) {
    if man.queued_action == Some(UIClickAction::ViewWorldList) {
        let samples = audio_serve.get(&core.audio).unwrap();
        audio.play(samples.get("click"));
        man.reset_ui();
        state.replace(GameState::ServerList).unwrap();
    }
}

pub fn ui_return_titlescreen(
    mut state: ResMut<State<GameState>>,
    mut man: ResMut<UIManager>,
    audio: Res<Audio>,
    core: Res<CoreAssets>,
    audio_serve: Res<Assets<AudioSamples>>,
) {
    if man.queued_action == Some(UIClickAction::GoToTitleScreen) {
        let samples = audio_serve.get(&core.audio).unwrap();
        audio.play(samples.get("click"));
        man.reset_ui();
        state.push(GameState::TitleScreen).unwrap();
    }
}

pub fn ui_open_settings(mut state: ResMut<State<GameState>>, mut man: ResMut<UIManager>) {
    if man.queued_action == Some(UIClickAction::OpenSettings) {
        man.reset_ui();
        state.push(GameState::Settings).unwrap();
    }
}
