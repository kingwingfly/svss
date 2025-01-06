use crate::{event::DoubleClickEvent, state::DoubleClickState};
use bevy::prelude::*;

pub fn double_click(
    time: Res<Time>,
    keys: Res<ButtonInput<MouseButton>>,
    mut click_state: Local<DoubleClickState>,
    mut evw_double_click: EventWriter<DoubleClickEvent>,
    mut q_window: Query<&mut Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    click_state.tick(time.delta());
    let mut btns = keys.get_just_pressed();
    loop {
        match click_state.click(btns.next().cloned()) {
            None => break,
            Some(btn) => {
                let window = q_window.single_mut();
                let (camera, camera_transform) = q_camera.single();
                let Some(window_pos) = window.cursor_position() else {
                    return;
                };
                let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, window_pos)
                else {
                    return;
                };
                evw_double_click.send(DoubleClickEvent { btn, world_pos });
            }
        }
    }
}
