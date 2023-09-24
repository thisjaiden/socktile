use crate::prelude::*;

pub fn text_input(
    mut tb: ResMut<crate::resources::TextBox>,
    mut char_evr: EventReader<ReceivedCharacter>,
    #[cfg(target_arch = "wasm32")] key_events: Res<Input<KeyCode>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        if key_events.just_pressed(KeyCode::Back) {
            tb.eat_buffer();
        }
        if key_events.just_pressed(KeyCode::Return) {
            tb.update_buffer('\n');
        }
    }
    for char in char_evr.iter() {
        tb.update_buffer(char.char);
        if char.char == '\r' {
            tb.eat_buffer();
            tb.update_buffer('\n');
        }
        // If we recieve a backspace character...
        if char.char == '\x08' || char.char == '\x7f' {
            // Remove backspace character
            tb.eat_buffer();
            // Remove one more (the actual backspace action)
            tb.eat_buffer();
        }
        // If we recieve an escape character...
        if char.char == '\x27' {
            // Remove the escape character
            tb.eat_buffer();
        }
    }
}

pub fn user_creation(
    mut commands: Commands,
    mut tb: ResMut<crate::resources::TextBox>,
    mut tb_q: Query<&mut Text, With<TextBox>>,
    mut netty: ResMut<Netty>,
    mut state: ResMut<NextState<GameState>>,
    mut disk: ResMut<Disk>,
    unloads: Query<Entity, With<RemoveOnStateChange>>,
    core: Res<CoreAssets>,
    lang_serve: Res<Assets<LanguageKeys>>,
) {
    let lang = lang_serve.get(&core.lang).unwrap();
    let text = tb_q.get_single_mut();
    if text.is_err() {
        return;
    }
    let mut text = text.unwrap();
    text.sections[0].value = tb.grab_buffer() + "";
    if tb.grab_buffer().contains('#') {
        text.sections[0].style.color = Color::RED;
        text.sections[1].value = lang.get("en_us.core.create_user.no_hashtags");
    }
    else if tb.grab_buffer().contains('/') || tb.grab_buffer().contains('\\') {
        text.sections[0].style.color = Color::RED;
        text.sections[1].value = lang.get("en_us.core.create_user.no_slashes");
    }
    else if tb.grab_buffer().starts_with(' ') {
        text.sections[0].style.color = Color::RED;
        text.sections[1].value = lang.get("en_us.core.create_user.space_start");
    }
    else if tb.grab_buffer().ends_with(' ') {
        text.sections[0].style.color = Color::RED;
        text.sections[1].value = lang.get("en_us.core.create_user.space_end");
    }
    else if tb.grab_buffer().chars().count() < 3 {
        text.sections[0].style.color = Color::RED;
        text.sections[1].value = lang.get("en_us.core.create_user.too_short");
        if tb.grab_buffer().contains('\n') {
            tb.eat_buffer();
        }
    }
    else if !tb.grab_buffer().is_empty() {
        // Reset text to normal if all is okay
        text.sections[1].value = String::from("\n ");
        text.sections[0].style.color = Color::BLACK;
        // Warnings for inconvenient but not disallowed names
        if tb.grab_buffer().chars().count() > 20 {
            text.sections[0].style.color = Color::ORANGE;
            text.sections[1].value = lang.get("en_us.core.create_user.too_long");
        }
        else if tb.grab_buffer().contains("  ") {
            text.sections[0].style.color = Color::ORANGE;
            text.sections[1].value = lang.get("en_us.core.create_user.double_space");
        }
        else if !tb.grab_buffer().is_ascii() {
            text.sections[0].style.color = Color::ORANGE;
            text.sections[1].value = lang.get("en_us.core.create_user.non_ascii");
        }
        if tb.grab_buffer().contains('\n') {
            let mut mode = tb.grab_buffer();
            mode = String::from(mode.trim_end());
            mode = String::from(mode.trim_end_matches('\n'));
            netty.send(Packet::CreateUser(User {
                username: mode.clone(),
                tag: 0,
            }));
            while !disk.update_user(User {
                username: mode.clone(),
                tag: 0,
            }) {}
            tb.clear_buffer();
            unloads.for_each(|e| {
                commands.entity(e).despawn();
            });
            state.set(GameState::TitleScreen);
        }
    }
}

pub fn game_creation(
    mut commands: Commands,
    mut tb: ResMut<crate::resources::TextBox>,
    mut tb_q: Query<(Entity, &mut Text), With<TextBox>>,
    uiman: Res<UIManager>,
    mut netty: ResMut<Netty>,
    mut state: ResMut<NextState<GameState>>,
    disk: Res<Disk>,
    materials: Res<AnimatorAssets>,
) {
    let (entity, mut text) = tb_q.single_mut();
    text.sections[0].value = tb.grab_buffer();
    if tb.grab_buffer().contains('#') || tb.grab_buffer().is_empty() {
        text.sections[0].style.color = Color::RED;
    }
    else {
        text.sections[0].style.color = Color::BLACK;
        if tb.grab_buffer().contains('\n') || uiman.queued_action == Some(UIClickAction::CreateWorld) {
            let mut mode = tb.grab_buffer();
            mode = String::from(mode.trim_end());
            mode = String::from(mode.trim_end_matches('\n'));
            netty.send(Packet::CreateWorld(mode));
            tb.clear_buffer();
            state.set(GameState::Play);
            commands.entity(entity).despawn_recursive();
            commands.spawn((
                SpriteBundle {
                    texture: materials.not_animated.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, PLAYER_CHARACTERS),
                    ..Default::default()
                },
                disk.user().unwrap(),
            ));
        }
    }
}

pub fn game_creation_once(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: String::new(),
                    style: TextStyle {
                        font: font_assets.simvoni.clone(),
                        font_size: 35.0,
                        color: Color::BLACK,
                    },
                }],
                alignment: TextAlignment::Center,
                linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter
            },
            transform: Transform::from_xyz(0.0, 0.0, UI_TEXT),
            ..Default::default()
        },
        TextBox {},
        RemoveOnStateChange {},
    ));
}
