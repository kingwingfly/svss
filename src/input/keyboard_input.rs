use crate::{event::TextRefreshEvent, state::TextInputState};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

pub fn ime_toggle(btns: Res<ButtonInput<KeyCode>>, mut text_input_state: ResMut<TextInputState>) {
    if text_input_state.target == Entity::PLACEHOLDER {
        return;
    }
    if !btns.just_pressed(KeyCode::ShiftLeft) {
        return;
    }
    for btn in btns.get_pressed() {
        if *btn != KeyCode::ShiftLeft {
            return;
        }
    }
    text_input_state.troggle_ime_state();
}

pub fn ime_input(
    mut cmds: Commands,
    mut evr_ime: EventReader<Ime>,
    mut text_input_state: ResMut<TextInputState>,
) {
    if text_input_state.target == Entity::PLACEHOLDER {
        return;
    }
    for ev in evr_ime.read() {
        let target = text_input_state.target;
        match ev {
            Ime::Commit { value, .. } => {
                debug!("IME confirmed text: {}", value);
                text_input_state.insert_str(value);
                cmds.trigger_targets(TextRefreshEvent::from(&*text_input_state), target);
            }
            Ime::Preedit {
                value,
                cursor: Some(cursor),
                ..
            } => {
                debug!("IME buffer: {:?}, cursor: {:?}", value, cursor);
                text_input_state.set_ime_buf(value, *cursor);
                cmds.trigger_targets(TextRefreshEvent::from(&*text_input_state), target);
            }
            Ime::Enabled { .. } => {
                debug!("IME mode enabled!");
            }
            Ime::Disabled { .. } => {
                debug!("IME mode disabled!");
            }
            _ => {}
        }
    }
}

pub fn text_input(
    mut cmds: Commands,
    mut text_input_state: ResMut<TextInputState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut key_evr: EventReader<KeyboardInput>,
    mut q_window: Query<&mut Window>,
) {
    if text_input_state.target == Entity::PLACEHOLDER {
        return;
    }
    let mut window = q_window.single_mut();
    window.ime_enabled = text_input_state.ime_state;
    window.ime_position = text_input_state.ime_position;
    let target = text_input_state.target;
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::Enter if keys.pressed(KeyCode::ShiftRight) => text_input_state.new_line(),
            KeyCode::Enter => {
                text_input_state.submit();
                cmds.trigger_targets(TextRefreshEvent::from(&*text_input_state), target);
                text_input_state.reset();
                debug!("input submit: {}", *text_input_state);
                window.ime_enabled = false;
                return;
            }
            KeyCode::ArrowLeft => text_input_state.move_left(),
            KeyCode::ArrowRight => text_input_state.move_right(),
            KeyCode::ArrowUp => text_input_state.move_up(),
            KeyCode::ArrowDown => text_input_state.move_down(),
            KeyCode::Backspace => text_input_state.backspace(),
            _ => {}
        }
    }
    if window.ime_enabled {
        cmds.trigger_targets(TextRefreshEvent::from(&*text_input_state), target);
        return;
    }
    for key in key_evr.read() {
        if key.state == ButtonState::Released {
            continue;
        }
        match &key.logical_key {
            Key::Character(c) if c.chars().all(|c| !c.is_control()) => {
                text_input_state.insert_str(c)
            }
            _ => {}
        }
        cmds.trigger_targets(TextRefreshEvent::from(&*text_input_state), target);
    }
}
