use crate::state::TextInputState;
use bevy::prelude::*;

const MOVE_SPEED: f32 = 50.0;

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .add_systems(Update, camera_movement);
    }
}

pub fn camera_setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
}

pub fn camera_movement(
    mut q_camera: Query<(&mut Camera2d, &mut Transform)>,
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
