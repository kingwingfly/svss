use bevy::prelude::*;
use std::fmt::Debug;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MouseClickEvent>()
            .add_systems(Update, (event));
    }
}

#[derive(Debug, Event)]
pub enum MouseClickEvent {
    SingleClick(MouseButton),
    DoubleClick(MouseButton),
    None,
}

fn event(
    mut cmds: Commands,
    mut evr: EventReader<MouseClickEvent>,
    mut windows: Query<&mut Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single_mut();
    let (camera, camera_transform) = q_camera.single();
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };
    for ev in evr.read() {
        debug!("{:?}", ev);
        match ev {
            MouseClickEvent::SingleClick(_) => {}
            MouseClickEvent::DoubleClick(mouse_button) => match mouse_button {
                MouseButton::Left => {
                    cmds.spawn((
                        Sprite {
                            color: Color::WHITE,
                            custom_size: Some((100., 100.).into()),
                            ..Default::default()
                        },
                        Transform::from_xyz(world_position.x, world_position.y, 0.),
                    ))
                    .observe(
                        |trigger: Trigger<Pointer<Over>>, mut sprite: Query<&mut Sprite>| {
                            if let Ok(mut s) = sprite.get_mut(trigger.target) {
                                s.color = Color::BLACK;
                            }
                        },
                    )
                    .observe(
                        |trigger: Trigger<Pointer<Out>>, mut sprite: Query<&mut Sprite>| {
                            if let Ok(mut s) = sprite.get_mut(trigger.target) {
                                s.color = Color::WHITE;
                            }
                        },
                    );
                }
                MouseButton::Right => {}
                MouseButton::Middle => {}
                MouseButton::Back => {}
                MouseButton::Forward => {}
                MouseButton::Other(_) => {}
            },
            MouseClickEvent::None => {}
        }
    }
}
