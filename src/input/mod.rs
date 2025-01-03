use crate::{
    event::{DoubleClickEvent, TextRefreshEvent},
    state::{DoubleClickState, KeyboardState},
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
        app.init_resource::<DoubleClickState>()
            .init_resource::<KeyboardState>()
            .add_systems(Update, (double_click, text_input));
    }
}

fn double_click(
    time: Res<Time>,
    mouse_input_events: Res<ButtonInput<MouseButton>>,
    mut click_state: ResMut<DoubleClickState>,
    mut double_click_evw: EventWriter<DoubleClickEvent>,
    mut windows: Query<&mut Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    click_state.tick(time.delta());
    let mut btns = mouse_input_events.get_just_pressed();
    loop {
        match click_state.click(btns.next().cloned()) {
            None => break,
            Some(btn) => {
                let window = windows.single_mut();
                let (camera, camera_transform) = q_camera.single();
                let Some(cursor) = window.cursor_position() else {
                    return;
                };
                let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor) else {
                    return;
                };
                double_click_evw.send(DoubleClickEvent { btn, world_pos });
            }
        }
    }
}

fn text_input(
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
