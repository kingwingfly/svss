use crate::{event::TextRefreshEvent, state::TextInputState};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

const KEY_CD_SECS: f32 = 0.01;

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

#[derive(Component, Deref, DerefMut, Debug)]
pub struct KeyCD(Timer);

impl Default for KeyCD {
    fn default() -> Self {
        Self(Timer::from_seconds(KEY_CD_SECS, TimerMode::Once))
    }
}

pub fn text_input(
    mut cmds: Commands,
    mut input_state: ResMut<TextInputState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut evr_keys: EventReader<KeyboardInput>,
    time: Res<Time>,
    mut key_cd: Local<KeyCD>,
) {
    const MODIFIERS1: [KeyCode; 2] = [KeyCode::ShiftLeft, KeyCode::ShiftRight];
    #[cfg(not(target_os = "macos"))]
    const MODIFIERS2: [KeyCode; 2] = [KeyCode::ControlLeft, KeyCode::ControlRight];
    #[cfg(target_os = "macos")]
    const MODIFIERS2: [KeyCode; 2] = [KeyCode::SuperLeft, KeyCode::SuperRight];
    if input_state.target == Entity::PLACEHOLDER {
        return;
    }
    if !key_cd.tick(time.delta()).finished() {
        return;
    }
    key_cd.reset();
    let target = input_state.target;
    for key in keys.get_pressed() {
        match key {
            KeyCode::Enter if keys.any_pressed(MODIFIERS1) => input_state.new_line(),
            KeyCode::Enter => {
                input_state.submit();
                cmds.trigger_targets(TextRefreshEvent::from(&*input_state), target);
                debug!("input submit: {}", *input_state);
                input_state.reset();
                return;
            }
            KeyCode::ArrowLeft if keys.any_pressed(MODIFIERS2) => input_state.move_to_line_head(),
            KeyCode::ArrowRight if keys.any_pressed(MODIFIERS2) => input_state.move_to_line_tail(),
            KeyCode::ArrowUp if keys.any_pressed(MODIFIERS2) => input_state.move_to_head(),
            KeyCode::ArrowDown if keys.any_pressed(MODIFIERS2) => input_state.move_to_tail(),
            KeyCode::ArrowLeft => input_state.move_left(),
            KeyCode::ArrowRight => input_state.move_right(),
            KeyCode::ArrowUp => input_state.move_up(),
            KeyCode::ArrowDown => input_state.move_down(),
            KeyCode::Backspace => input_state.backspace(),
            KeyCode::Space => input_state.insert_str(" "),
            _ => {}
        }
    }
    for key in evr_keys.read() {
        if key.state == ButtonState::Released {
            continue;
        }
        match &key.logical_key {
            Key::Character(c) if c.chars().all(|c| !c.is_control()) => input_state.insert_str(c),
            _ => {}
        }
        cmds.trigger_targets(TextRefreshEvent::from(&*input_state), target);
    }
}
