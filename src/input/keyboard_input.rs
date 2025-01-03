use crate::{event::TextRefreshEvent, state::KeyboardState};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

pub fn text_input(
    mut cmds: Commands,
    mut keyboard_state: ResMut<KeyboardState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut key_evr: EventReader<KeyboardInput>,
) {
    let target = keyboard_state.target;
    if target == Entity::PLACEHOLDER {
        return;
    }
    for key in key_evr.read() {
        if key.state == ButtonState::Released {
            continue;
        }
        match &key.logical_key {
            Key::Enter if keys.pressed(KeyCode::ShiftRight) => keyboard_state.new_line(),
            Key::Enter => {
                debug!("input submit: {}", *keyboard_state);
                cmds.trigger_targets(TextRefreshEvent(keyboard_state.reset()), target);
                return;
            }
            Key::Backspace => keyboard_state.backspace(),
            Key::ArrowLeft => keyboard_state.move_left(),
            Key::ArrowRight => keyboard_state.move_right(),
            Key::ArrowUp => keyboard_state.move_up(),
            Key::ArrowDown => keyboard_state.move_down(),
            Key::Character(c) if c.chars().all(|c| !c.is_control()) => keyboard_state.insert_str(c),
            _ => {}
        }
        cmds.trigger_targets(TextRefreshEvent(keyboard_state.to_string()), target);
    }
}
