use crate::{camera::PrimaryCamera, state::PickState};
use bevy::{color::palettes::css::*, prelude::*};

pub fn left_pick(
    mut gizmos: Gizmos,
    btn: Res<ButtonInput<MouseButton>>,
    pick_state: Res<PickState>,
    mut start: Local<Option<Vec2>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    if !btn.pressed(MouseButton::Left) || !pick_state.active {
        *start = None;
        return;
    }
    let window = q_window.single();
    let Some(pos) = window.cursor_position() else {
        return;
    };
    let (camera, camera_transform) = q_camera.single();
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) else {
        return;
    };
    match *start {
        Some(start) => {
            if start.x > world_pos.x {
                gizmos.rect_2d((start + world_pos) / 2., (start - world_pos).abs(), WHITE);
            }
        }
        None => *start = Some(world_pos),
    }
}

pub fn right_pick(
    mut gizmos: Gizmos,
    btn: Res<ButtonInput<MouseButton>>,
    pick_state: Res<PickState>,
    mut start: Local<Option<Vec2>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    if !btn.pressed(MouseButton::Left) || !pick_state.active {
        *start = None;
        return;
    }
    let window = q_window.single();
    let Some(pos) = window.cursor_position() else {
        return;
    };
    let (camera, camera_transform) = q_camera.single();
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) else {
        return;
    };
    match *start {
        Some(start) => {
            if start.x < world_pos.x {
                gizmos.rect_2d((start + world_pos) / 2., (start - world_pos).abs(), WHITE);
            }
        }
        None => *start = Some(world_pos),
    }
}
