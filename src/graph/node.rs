use crate::{
    camera::PrimaryCamera,
    event::{CreateNodeEvent, EditEvent, TextRefreshEvent},
    state::{DoubleClickState, PickState, TextInputState},
};
use bevy::{color::palettes::css::*, prelude::*};
use bevy_quadtree::CollisionRect;

const FONT_WIDTH: f32 = 18.0;
const FONT_HEIGHT: f32 = FONT_WIDTH * 1.2;
const CUSTOM_SIZE: Vec2 = Vec2::new(FONT_WIDTH, FONT_HEIGHT * 2.);

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TextRefreshEvent>()
            .add_systems(Update, (node_create,));
    }
}

fn node_create(
    mut cmds: Commands,
    mut evr_double_click: EventReader<CreateNodeEvent>,
    asset_server: Res<AssetServer>,
    mut text_input_state: ResMut<TextInputState>,
    mut q_window: Query<&mut Window>,
) {
    for ev in evr_double_click.read() {
        // text input target change
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
            CollisionRect::from(Rect::from_center_size(ev.world_pos, CUSTOM_SIZE)),
            Transform::from_xyz(ev.world_pos.x, ev.world_pos.y, 1.),
        ))
        .observe(
            |trigger: Trigger<Pointer<Drag>>,
             mut q_sprite: Query<&mut Transform, With<Sprite>>,
             q_projection: Query<&Projection, With<PrimaryCamera>>,
             mut pick_state: ResMut<PickState>| {
                if trigger.button == PointerButton::Primary {
                    pick_state.active = false;
                    if let Projection::Orthographic(projection) = q_projection.single() {
                        if let Ok(mut transform) = q_sprite.get_mut(trigger.entity()) {
                            transform.translation.x += trigger.event().delta.x * projection.scale;
                            transform.translation.y -= trigger.event().delta.y * projection.scale;
                        }
                    };
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Click>>,
             mut cmds: Commands,
             q_children: Query<&Children, With<Sprite>>,
             q_text: Query<&Text2d>,
             text_input_state: ResMut<TextInputState>,
             mut double_click_state: Local<DoubleClickState>| {
                if double_click_state.click(Some(trigger.button)) == Some(PointerButton::Primary) {
                    if let Ok(children) = q_children.get(trigger.entity()) {
                        for &e in children {
                            if q_text.contains(e) {
                                if e == text_input_state.target {
                                    return;
                                }
                                cmds.trigger_targets(EditEvent, e);
                                return;
                            }
                        }
                    }
                }
            },
        )
        .observe(
            |trigger: Trigger<TextRefreshEvent>,
             mut q_box: Query<&mut Sprite>,
             mut q_window: Query<&mut Window>| {
                let mut window = q_window.single_mut();
                let Some(window_pos) = window.cursor_position() else {
                    return;
                };
                if let Ok(mut s) = q_box.get_mut(trigger.entity()) {
                    let ev = trigger.event();
                    let delta = Vec2::new(ev.width * FONT_WIDTH, (ev.height - 1.) * FONT_HEIGHT);
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
                        font_size: FONT_WIDTH,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextColor(BLUE.into()),
                    Transform::from_xyz(0., 0., 2.),
                ))
                .observe(
                    |trigger: Trigger<TextRefreshEvent>, mut q_text: Query<&mut Text2d>| {
                        let ev = trigger.event();
                        if let Ok(mut t) = q_text.get_mut(trigger.entity()) {
                            t.0 = ev.text.clone();
                        }
                    },
                )
                .observe(
                    |trigger: Trigger<EditEvent>,
                     mut cmds: Commands,
                     q_text: Query<&Text2d>,
                     mut text_input_state: ResMut<TextInputState>| {
                        if let Ok(t) = q_text.get(trigger.entity()) {
                            text_input_state.submit();
                            cmds.trigger_targets(
                                TextRefreshEvent::from(&*text_input_state),
                                text_input_state.target,
                            );
                            text_input_state.reset();
                            text_input_state.input_buf =
                                t.0.split("\n").map(|line| line.chars().collect()).collect();
                            text_input_state.target = trigger.entity();
                            cmds.trigger_targets(
                                TextRefreshEvent::from(&*text_input_state),
                                text_input_state.target,
                            );
                        }
                    },
                )
                .id();
        });
    }
}
