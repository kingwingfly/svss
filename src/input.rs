use crate::{
    event::{ClickEvent, TextRefresh},
    state::{ClickState, KeyboardState},
};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClickState>()
            .init_resource::<KeyboardState>()
            .add_systems(Update, (click, keyboard));
    }
}

fn click(
    time: Res<Time>,
    mouse_input_events: Res<ButtonInput<MouseButton>>,
    mut click_state: ResMut<ClickState>,
    mut evw: EventWriter<ClickEvent>,
) {
    click_state.tick(time.delta());
    let mut btns = mouse_input_events.get_just_pressed();
    loop {
        match click_state.click(btns.next().cloned()) {
            ClickEvent::None => break,
            ev => {
                evw.send(ev);
            }
        }
    }
}

fn keyboard(
    mut cmds: Commands,
    mut keyboard_state: ResMut<KeyboardState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut key_evr: EventReader<KeyboardInput>,
) {
    let target = keyboard_state.target;
    if let Some(s) = keyboard_state.input_buf.as_mut() {
        for key in key_evr.read() {
            if key.state == ButtonState::Released {
                continue;
            }
            match &key.logical_key {
                Key::Enter if keys.pressed(KeyCode::ShiftRight) => {
                    s.push('\n');
                }
                Key::Enter => {
                    debug!("input submit: {}", s);
                    cmds.trigger_targets(TextRefresh::Finish(s.to_owned()), target);
                    keyboard_state.input_buf = None;
                    return;
                }
                Key::Backspace => {
                    s.pop();
                }
                Key::Character(c) if c.chars().all(|c| !c.is_control()) => {
                    s.push_str(c);
                }
                _ => {}
            }
            cmds.trigger_targets(TextRefresh::Inputing(s.clone()), target);
        }
    }
}
