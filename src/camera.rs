use crate::state::TextInputState;
use bevy::{
    input::{gestures::PinchGesture, mouse::MouseWheel},
    prelude::*,
};

const MOVE_SPEED: f32 = 50.0;

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .add_systems(Update, (camera_movement,));
        app.add_systems(Update, (camera_scale,));
    }
}

pub fn camera_setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
}

pub fn camera_movement(
    mut q_camera: Query<(&Camera2d, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    text_input_state: Res<TextInputState>,
) {
    if text_input_state.target != Entity::PLACEHOLDER {
        return;
    }
    let (_, mut transform) = q_camera.single_mut();
    if keyboard.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        transform.translation.x -= MOVE_SPEED;
    } else if keyboard.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        transform.translation.x += MOVE_SPEED;
    } else if keyboard.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        transform.translation.y += MOVE_SPEED;
    } else if keyboard.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        transform.translation.y -= MOVE_SPEED;
    }
}

#[cfg(target_os = "macos")]
pub fn camera_scale(
    mut q_camera: Query<(&Camera2d, &mut Transform)>,
    mut gesture_evr: EventReader<PinchGesture>,
) {
    let (_, mut transform) = q_camera.single_mut();
    for ev in gesture_evr.read() {
        transform.scale = (transform.scale - ev.0).max(Vec3::splat(0.125));
    }
}

#[cfg(not(target_os = "macos"))]
pub fn camera_scale(
    mut q_camera: Query<(&Camera2d, &mut Transform)>,
    mut mouse_wheel_evr: EventReader<MouseWheel>,
) {
    let (_, mut transform) = q_camera.single_mut();
    for ev in mouse_wheel_evr.read() {
        transform.scale = (transform.scale - ev.y / 50.).max(Vec3::splat(0.125));
    }
}
