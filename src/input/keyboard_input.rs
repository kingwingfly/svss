use crate::{event::TextRefreshEvent, state::TextInputState};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

pub fn ime_toggle(
    btns: Res<ButtonInput<KeyCode>>,
    text_input_state: ResMut<TextInputState>,
    mut q_window: Query<&mut Window>,
    mut possiable: Local<bool>,
) {
    if text_input_state.target == Entity::PLACEHOLDER {
        return;
    }
    if btns.just_pressed(KeyCode::ShiftLeft) {
        *possiable = true;
    }
    for btn in btns.get_just_pressed() {
        if *btn != KeyCode::ShiftLeft {
            *possiable = false;
            return;
        }
    }
    if *possiable && btns.just_released(KeyCode::ShiftLeft) {
        if text_input_state.ime_buf.is_empty() {
            let mut window = q_window.single_mut();
            window.ime_enabled = !window.ime_enabled;
        }
        *possiable = false;
    }
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
    mut evr_keys: EventReader<KeyboardInput>,
    time: Res<Time>,
) {
    if text_input_state.target == Entity::PLACEHOLDER {
        return;
    }
    if !text_input_state.key_cd.tick(time.delta()).finished() {
        return;
    }
    text_input_state.key_cd.reset();
    let target = text_input_state.target;
    for key in keys.get_pressed() {
        match key {
            KeyCode::Enter if keys.pressed(KeyCode::ShiftRight) => text_input_state.new_line(),
            KeyCode::Enter => {
                text_input_state.submit();
                cmds.trigger_targets(TextRefreshEvent::from(&*text_input_state), target);
                debug!("input submit: {}", *text_input_state);
                text_input_state.reset();
                return;
            }
            KeyCode::ArrowLeft => text_input_state.move_left(),
            KeyCode::ArrowRight => text_input_state.move_right(),
            KeyCode::ArrowUp => text_input_state.move_up(),
            KeyCode::ArrowDown => text_input_state.move_down(),
            KeyCode::Backspace => text_input_state.backspace(),
            KeyCode::Space => text_input_state.insert_str(" "),
            _ => {}
        }
    }
    for key in evr_keys.read() {
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
