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
    mut double_click_evr: EventReader<DoubleClickEvent>,
    asset_server: Res<AssetServer>,
    mut text_input_state: ResMut<TextInputState>,
) {
    for ev in double_click_evr.read() {
        debug!("{:?}", ev);
        match ev.btn {
            MouseButton::Left => {
                let window_pos = ev.window_pos;
                text_input_state.ime_position = window_pos;
                // double click leads text input target change
                if text_input_state.target != Entity::PLACEHOLDER {
                    let target = text_input_state.target;
                    text_input_state.submit();
                    cmds.trigger_targets(TextRefreshEvent::from(&*text_input_state), target);
                    text_input_state.reset();
                }
                let mut sprite_cmds = cmds.spawn((
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(CUSTOM_SIZE),
                        ..Default::default()
                    },
                    Transform::from_xyz(ev.world_pos.x, ev.world_pos.y, 0.),
                ));
                let sprite_id = sprite_cmds.id();
                sprite_cmds.observe(
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
                );
                let target_mut = &mut text_input_state.target;
                let asset_server_ref = &asset_server;
                sprite_cmds.with_children(move |p| {
                    let mut entity_cmds = p.spawn((
                        Text2d::new("|"),
                        TextFont {
                            font: asset_server_ref.load("fonts/PingFangSC-Regular.otf"),
                            font_size: FONT_WIDTH,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor(BLUE.into()),
                        Transform::from_xyz(0., 0., 1.),
                    ));
                    *target_mut = entity_cmds.id();
                    entity_cmds.observe(
                        move |trigger: Trigger<TextRefreshEvent>,
                              mut q_text: Query<&mut Text2d>,
                              mut q_sprite: Query<&mut Sprite>,
                              mut text_input_state: ResMut<TextInputState>,| {
                            let ev = trigger.event();
                            if let Ok(mut t) = q_text.get_mut(trigger.entity()) {
                                t.0 = ev.text.clone();
                            }
                            if let Ok(mut s) = q_sprite.get_mut(sprite_id) {
                                let offset = Vec2::new(
                                    ev.width * FONT_WIDTH,
                                    (ev.height - 1.) * FONT_HEIGHT,
                                );
                                text_input_state.ime_position = window_pos + offset;
                                s.custom_size = Some(CUSTOM_SIZE + offset);
                            }
                        },
                    );
                });
            }
            MouseButton::Right => {}
            _ => {}
        }
    }
}
