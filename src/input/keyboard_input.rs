use crate::{event::TextRefreshEvent, state::TextInputState};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

pub fn text_input(
    mut cmds: Commands,
    mut text_input_state: ResMut<TextInputState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut key_evr: EventReader<KeyboardInput>,
) {
    let target = text_input_state.target;
    if target == Entity::PLACEHOLDER {
        return;
    }
    for key in key_evr.read() {
        if key.state == ButtonState::Released {
            continue;
        }
        match &key.logical_key {
            Key::Enter if keys.pressed(KeyCode::ShiftRight) => text_input_state.new_line(),
            Key::Enter => {
                debug!("input submit: {}", *text_input_state);
                cmds.trigger_targets(TextRefreshEvent(text_input_state.reset()), target);
                return;
            }
            Key::Backspace => text_input_state.backspace(),
            Key::ArrowLeft => text_input_state.move_left(),
            Key::ArrowRight => text_input_state.move_right(),
            Key::ArrowUp => text_input_state.move_up(),
            Key::ArrowDown => text_input_state.move_down(),
            Key::Character(c) if c.chars().all(|c| !c.is_control()) => {
                text_input_state.insert_str(c)
            }
            _ => {}
        }
        cmds.trigger_targets(TextRefreshEvent(text_input_state.to_string()), target);
    }
}
