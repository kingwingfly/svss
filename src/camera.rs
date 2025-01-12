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
    cmds.spawn((
        Camera2d,
        Projection::from(OrthographicProjection::default_2d()),
        PrimaryCamera,
    ));
}

fn camera_movement(
    mut q_camera: Query<&mut Transform, With<PrimaryCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    text_input_state: Res<TextInputState>,
) {
    if text_input_state.target != Entity::PLACEHOLDER {
        return;
    }
    let mut camera = q_camera.single_mut();
    if keyboard.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        camera.translation.x -= MOVE_SPEED;
    } else if keyboard.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        camera.translation.x += MOVE_SPEED;
    } else if keyboard.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        camera.translation.y += MOVE_SPEED;
    } else if keyboard.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        camera.translation.y -= MOVE_SPEED;
    } else if keyboard.just_pressed(KeyCode::Space) {
        camera.translation = Vec3::ZERO;
    }
}

#[cfg(target_os = "macos")]
fn camera_scale(
    mut q_camera: Query<&mut Projection, With<PrimaryCamera>>,
    mut gesture_evr: EventReader<PinchGesture>,
    keyboard: Res<ButtonInput<KeyCode>>,
    text_input_state: Res<TextInputState>,
) {
    let mut projection = q_camera.single_mut();
    let Projection::Orthographic(ref mut projection) = projection.as_mut() else {
        unreachable!(
            "The `Projection` component was explicitly built with `Projection::Orthographic`"
        );
    };
    for ev in gesture_evr.read() {
        projection.scale = (projection.scale - ev.0).clamp(0.1, 5.);
    }
    if keyboard.just_pressed(KeyCode::Space) && text_input_state.target != Entity::PLACEHOLDER {
        projection.scale = 1.;
    }
}

#[cfg(not(target_os = "macos"))]
fn camera_scale(
    mut q_camera: Query<&mut Projection, With<PrimaryCamera>>,
    mut mouse_wheel_evr: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    text_input_state: Res<TextInputState>,
) {
    let mut projection = q_camera.single_mut();
    let Projection::Orthographic(ref mut projection) = projection.as_mut() else {
        unreachable!(
            "The `Projection` component was explicitly built with `Projection::Orthographic`"
        );
    };
    for ev in mouse_wheel_evr.read() {
        projection.scale = (projection.scale - ev.y).clamp(0.1, 5.);
    }
    if keyboard.just_pressed(KeyCode::Space) && text_input_state.target != Entity::PLACEHOLDER {
        projection.scale = 1.;
    }
}
