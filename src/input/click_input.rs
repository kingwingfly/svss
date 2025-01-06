use crate::{event::DoubleClickEvent, state::DoubleClickState};
use bevy::prelude::*;

pub fn double_click(
    time: Res<Time>,
    mouse_input_events: Res<ButtonInput<MouseButton>>,
    mut click_state: Local<DoubleClickState>,
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
                let Some(window_pos) = window.cursor_position() else {
                    return;
                };
                let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, window_pos)
                else {
                    return;
                };
                double_click_evw.send(DoubleClickEvent {
                    btn,
                    world_pos,
                    window_pos,
                });
            }
        }
    }
}
