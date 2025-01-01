use crate::state::KeyboardState;
use bevy::{color::palettes::css::*, prelude::*};

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickEvent>()
            .add_event::<TextRefresh>()
            .add_systems(Update, (click_event,));
    }
}

#[derive(Debug, Event)]
pub enum ClickEvent {
    SingleClick(MouseButton),
    DoubleClick(MouseButton),
    None,
}

#[derive(Debug, Event)]
pub enum TextRefresh {
    Inputing(String),
    Finish(String),
}

fn click_event(
    mut cmds: Commands,
    mut evr: EventReader<ClickEvent>,
    mut windows: Query<&mut Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    asset_server: Res<AssetServer>,
    mut keyboard_state: ResMut<KeyboardState>,
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
            ClickEvent::SingleClick(_) => {}
            ClickEvent::DoubleClick(mouse_button) => match mouse_button {
                MouseButton::Left => {
                    if keyboard_state.target != Entity::PLACEHOLDER {
                        if let Some(s) = keyboard_state.input_buf.as_ref() {
                            cmds.trigger_targets(
                                TextRefresh::Finish(s.to_owned()),
                                keyboard_state.target,
                            );
                        }
                    }
                    cmds.spawn((
                        Sprite {
                            color: Color::WHITE,
                            custom_size: Some((100., 100.).into()),
                            ..Default::default()
                        },
                        Transform::from_xyz(world_position.x, world_position.y, 0.),
                    ))
                    .with_children(|p| {
                        let mut entity_cmds = p.spawn((
                            Text2d::new("|"),
                            TextFont {
                                font: asset_server.load("fonts/FiraCode-Retina.ttf"),
                                font_size: 18.0,
                                ..default()
                            },
                            TextLayout::new_with_justify(JustifyText::Center),
                            TextColor(BLUE.into()),
                            Transform::from_xyz(0., 0., 1.),
                        ));
                        keyboard_state.input_buf = Some(String::new());
                        keyboard_state.target = entity_cmds.id();
                        entity_cmds.observe(
                            move |trigger: Trigger<TextRefresh>, mut text: Query<&mut Text2d>| {
                                if let Ok(mut t) = text.get_mut(trigger.entity()) {
                                    match trigger.event() {
                                        TextRefresh::Inputing(s) => {
                                            t.0 = format!("{s}|");
                                        }
                                        TextRefresh::Finish(s) => {
                                            t.0 = s.to_owned();
                                        }
                                    }
                                }
                            },
                        );
                    });
                }
                MouseButton::Right => {}
                _ => {}
            },
            ClickEvent::None => {}
        }
    }
}
