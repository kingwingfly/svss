use crate::{
    event::{DoubleClickEvent, TextRefreshEvent},
    state::TextInputState,
};
use bevy::{color::palettes::css::*, prelude::*};

const FONT_WIDTH: f32 = 18.0;
const FONT_HEIGHT: f32 = FONT_WIDTH * 1.2;
const CUSTOM_SIZE: Vec2 = Vec2::new(FONT_WIDTH, FONT_HEIGHT * 2.);

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DoubleClickEvent>()
            .add_event::<TextRefreshEvent>()
            .add_systems(Update, (node_create,));
    }
}

fn node_create(
    mut cmds: Commands,
    mut evr_double_click: EventReader<DoubleClickEvent>,
    asset_server: Res<AssetServer>,
    mut text_input_state: ResMut<TextInputState>,
    mut q_window: Query<&mut Window>,
) {
    for ev in evr_double_click.read() {
        debug!("{:?}", ev);
        match ev.btn {
            MouseButton::Left => {
                // double click leads text input target change
                if text_input_state.target != Entity::PLACEHOLDER {
                    text_input_state.submit();
                    cmds.trigger_targets(
                        TextRefreshEvent::from(&*text_input_state),
                        text_input_state.target,
                    );
                    text_input_state.reset();
                }
                let mut window = q_window.single_mut();
                let Some(window_pos) = window.cursor_position() else {
                    return;
                };
                window.ime_position = window_pos;
                cmds.spawn((
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(CUSTOM_SIZE),
                        ..Default::default()
                    },
                    Transform::from_xyz(ev.world_pos.x, ev.world_pos.y, 0.),
                ))
                .observe(
                    |trigger: Trigger<Pointer<Drag>>,
                     mut q: ParamSet<(
                        Query<&mut Transform, With<Sprite>>,
                        Query<&Transform, With<Camera2d>>,
                    )>| {
                        let scale = q.p1().single().scale;
                        if let Ok(mut transform) = q.p0().get_mut(trigger.entity()) {
                            transform.translation.x += trigger.event().delta.x * scale.x;
                            transform.translation.y -= trigger.event().delta.y * scale.y;
                        }
                    },
                )
                .observe(
                    |trigger: Trigger<Pointer<Click>>,
                     mut cmds: Commands,
                     q_children: Query<(&Sprite, &Children)>,
                     q_text: Query<&Text2d>,
                     mut text_input_state: ResMut<TextInputState>| {
                        if let Ok((_, children)) = q_children.get(trigger.entity()) {
                            for &e in children {
                                if let Ok(t) = q_text.get(e) {
                                    if e == text_input_state.target {
                                        return;
                                    }
                                    text_input_state.submit();
                                    cmds.trigger_targets(
                                        TextRefreshEvent::from(&*text_input_state),
                                        text_input_state.target,
                                    );
                                    text_input_state.reset();
                                    text_input_state.input_buf =
                                        t.0.split("\n")
                                            .map(|line| line.chars().collect())
                                            .collect();
                                    text_input_state.target = e;
                                    cmds.trigger_targets(
                                        TextRefreshEvent::from(&*text_input_state),
                                        text_input_state.target,
                                    );
                                    break;
                                }
                            }
                        }
                    },
                )
                .observe(
                    |trigger: Trigger<TextRefreshEvent>,
                     mut q_sprite: Query<&mut Sprite>,
                     mut q_window: Query<&mut Window>| {
                        let mut window = q_window.single_mut();
                        let Some(window_pos) = window.cursor_position() else {
                            return;
                        };
                        if let Ok(mut s) = q_sprite.get_mut(trigger.entity()) {
                            let ev = trigger.event();
                            let delta =
                                Vec2::new(ev.width * FONT_WIDTH, (ev.height - 1.) * FONT_HEIGHT);
                            s.custom_size = Some(CUSTOM_SIZE + delta);
                            window.ime_position = window_pos + delta;
                        }
                    },
                )
                .with_children(|p| {
                    text_input_state.target = p
                        .spawn((
                            Text2d::new("|"),
                            TextFont {
                                font: asset_server.load("fonts/SourceHanSansCN-Regular.otf"),
                                font_size: FONT_HEIGHT,
                                ..default()
                            },
                            TextLayout::new_with_justify(JustifyText::Center),
                            TextColor(BLUE.into()),
                            Transform::from_xyz(0., 0., 1.),
                        ))
                        .observe(
                            |trigger: Trigger<TextRefreshEvent>, mut q_text: Query<&mut Text2d>| {
                                let ev = trigger.event();
                                if let Ok(mut t) = q_text.get_mut(trigger.entity()) {
                                    t.0 = ev.text.clone();
                                }
                            },
                        )
                        .id();
                });
            }
            MouseButton::Right => {}
            _ => {}
        }
    }
}
