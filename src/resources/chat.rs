use crate::prelude::*;
use super::{Reality, reality::MenuState, TextBox};

#[derive(Resource)]
pub struct Chat {
    pub history: Vec<ChatMessage>,
    pub is_chat_open: MenuState
}

impl Chat {
    pub fn init() -> Chat {
        Chat {
            history: vec![],
            is_chat_open: MenuState::Closed
        }
    }
    pub fn is_open(&self) -> bool {
        self.is_chat_open != MenuState::Closed
    }
    pub fn queue_open(&mut self) {
        self.is_chat_open = MenuState::Queued;
    }
    pub fn escape_close(&mut self) {
        // Close chat immediately, without sending message.
        self.is_chat_open = MenuState::Closed;
    }
    fn add_message(&mut self, msg: ChatMessage) {
        self.history.push(msg);
        self.history.sort_by(|a, b| a.sent_at.elapsed().cmp(&b.sent_at.elapsed()));
        while self.history.len() > 9 {
            self.history.pop();
        }
    }
    pub fn system_open_chat(
        mut selfs: ResMut<Chat>,
        mut tb: ResMut<TextBox>
    ) {
        if selfs.is_chat_open == MenuState::Queued {
            tb.clear_buffer();
            selfs.is_chat_open = MenuState::Open;
        }
    }
    pub fn system_type_chat(
        selfs: Res<Chat>,
        mut tb: ResMut<TextBox>,
        mut boxes: Query<(&mut Text, &ChatBox)>
    ) {
        if selfs.is_chat_open == MenuState::Open {
            boxes.for_each_mut(|(mut text, box_)| {
                if box_.location == 0 {
                    text.sections[0].value = tb.grab_buffer();
                }
            });
        }
    }
    pub fn system_send_chat(
        mut selfs: ResMut<Chat>,
        mut tb: ResMut<TextBox>,
        mut netty: ResMut<Netty>,
        mut boxes: Query<(&mut Text, &ChatBox)>,
        disk: Res<Disk>,
        keys: Res<Input<KeyCode>>
    ) {
        if selfs.is_chat_open == MenuState::Open && keys.just_pressed(disk.control_config().send_chat) {
            boxes.for_each_mut(|(mut text, box_)| {
                if box_.location == 0 {
                    text.sections[0].value = String::new();
                    netty.send(Packet::SendChatMessage(ChatMessage {
                        text: tb.grab_buffer().trim_end_matches('\n').trim_end_matches('\r').to_string(),
                        color: Color::BLACK,
                        sent_at: std::time::Instant::now()
                    }));
                    tb.clear_buffer();
                    selfs.is_chat_open = MenuState::Closed;
                }
            });
        }
    }
    pub fn system_init(
        mut commands: Commands,
        fonts: ResMut<FontAssets>
    ) {
        for index in 0..10 {
            commands.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: String::new(),
                                style: TextStyle {
                                    font: fonts.apple_tea.clone(),
                                    font_size: 32.0,
                                    color: Color::BLACK
                                }
                            }
                        ],
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Left
                        }
                    },
                    transform: Transform::from_xyz(-(1920.0 / 2.0), -(1080.0 / 2.0) + 12.0 + (40.0 * index as f32), UI_TEXT),
                    ..Default::default()
                },
                ChatBox { location: index },
                UILocked {}
            ));
        }
    }
    pub fn system_display_chat(
        selfs: Res<Chat>,
        mut boxes: Query<(&mut Text, &ChatBox, &mut Transform)>
    ) {
        boxes.for_each_mut(|(mut text, thisbox, mut loc)| {
            loc.translation.x = -(1920.0 / 2.0);
            loc.translation.y = -(1080.0 / 2.0) + 12.0 + (40.0 * thisbox.location as f32);
            if thisbox.location < selfs.history.len() + 1 && thisbox.location != 0 {
                let thismsg = &selfs.history[thisbox.location - 1];
                let mut iso_color = thismsg.color;
                let midalpha = iso_color.a() - (0.01 * thismsg.sent_at.elapsed().as_secs_f32());
                if midalpha < 0.0 {
                    iso_color.set_a(0.0);
                }
                else {
                    iso_color.set_a(midalpha);
                }
                if selfs.is_chat_open == MenuState::Open {
                    iso_color.set_a(1.0);
                }
                text.sections[0].value = thismsg.text.clone();
                text.sections[0].style.color = iso_color;
            }
        });
    }
    pub fn system_pull_messages(
        mut selfs: ResMut<Chat>,
        mut reality: ResMut<Reality>
    ) {
        for message in reality.pull_messages() {
            selfs.add_message(message);
        }
    }
}

use std::time::Instant;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub text: String,
    pub color: Color,
    #[serde(skip)]
    #[serde(default = "Instant::now")]
    pub sent_at: Instant
}
