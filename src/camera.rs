use crate::state::TextInputState;
#[cfg(target_os = "macos")]
use bevy::input::gestures::PinchGesture;
#[cfg(not(target_os = "macos"))]
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

const MOVE_SPEED: f32 = 32.0;

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .add_systems(Update, (camera_movement,));
        app.add_systems(Update, (camera_scale,));
    }
}

#[derive(Component)]
pub struct PrimaryCamera;

fn camera_setup(mut cmds: Commands) {
    cmds.spawn((Camera2d, PrimaryCamera));
}

fn camera_movement(
    mut q_camera: Query<&mut Transform, With<PrimaryCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    text_input_state: Res<TextInputState>,
) {
    if text_input_state.target != Entity::PLACEHOLDER {
        return;
    }
    let mut transform = q_camera.single_mut();
    if keyboard.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        transform.translation.x -= MOVE_SPEED;
    } else if keyboard.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        transform.translation.x += MOVE_SPEED;
    } else if keyboard.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        transform.translation.y += MOVE_SPEED;
    } else if keyboard.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        transform.translation.y -= MOVE_SPEED;
    } else if keyboard.just_pressed(KeyCode::Space) {
        transform.translation = Vec3::ZERO;
        transform.scale = Vec3::ONE;
    }
}

#[cfg(target_os = "macos")]
fn camera_scale(
    mut q_camera: Query<&mut Transform, With<PrimaryCamera>>,
    mut gesture_evr: EventReader<PinchGesture>,
) {
    let mut transform = q_camera.single_mut();
    for ev in gesture_evr.read() {
        transform.scale = (transform.scale - ev.0).clamp(Vec3::splat(0.125), Vec3::splat(2.));
    }
}

#[cfg(not(target_os = "macos"))]
fn camera_scale(
    mut q_camera: Query<&mut Transform, With<PrimaryCamera>>,
    mut mouse_wheel_evr: EventReader<MouseWheel>,
) {
    let mut transform = q_camera.single_mut();
    for ev in mouse_wheel_evr.read() {
        transform.scale =
            (transform.scale - (ev.y / 8.)).clamp(Vec3::splat(0.125), Vec3::splat(2.));
    }
}
