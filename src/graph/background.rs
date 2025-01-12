use crate::{camera::PrimaryCamera, event::CreateNodeEvent, state::DoubleClickState};
use bevy::prelude::*;

const BACKGROUND_SIZE: Vec2 = Vec2::new(10000., 10000.);

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background);
    }
}

fn setup_background(mut cmds: Commands) {
    cmds.spawn((
        Sprite {
            color: Color::BLACK,
            custom_size: Some(BACKGROUND_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
    ))
    .observe(
        |trigger: Trigger<Pointer<Click>>,
         mut evw_double_click: EventWriter<CreateNodeEvent>,
         mut q_window: Query<&mut Window>,
         q_camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
         mut double_click_state: Local<DoubleClickState>| {
            if double_click_state.click(Some(trigger.button)) == Some(PointerButton::Primary) {
                let window = q_window.single_mut();
                let (camera, camera_transform) = q_camera.single();
                let Some(window_pos) = window.cursor_position() else {
                    return;
                };
                let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, window_pos)
                else {
                    return;
                };
                evw_double_click.send(CreateNodeEvent { world_pos });
            }
        },
    )
    .observe(
        |trigger: Trigger<Pointer<Drag>>,
         mut q_camera: Query<&mut Transform, With<PrimaryCamera>>| {
            if trigger.button == PointerButton::Secondary {
                let mut transform = q_camera.single_mut();
                transform.translation.x -= trigger.delta.x * transform.scale.x;
                transform.translation.y += trigger.delta.y * transform.scale.y;
            }
        },
    );
}
