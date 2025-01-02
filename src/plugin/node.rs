use crate::{
    event::{DoubleClickEvent, TextRefreshEvent},
    state::KeyboardState,
};
use bevy::{color::palettes::css::*, prelude::*};

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DoubleClickEvent>()
            .add_event::<TextRefreshEvent>()
            .add_systems(Update, (node_create_sys,));
    }
}

fn node_create_sys(
    mut cmds: Commands,
    mut double_click_evr: EventReader<DoubleClickEvent>,
    asset_server: Res<AssetServer>,
    // double click leads text input target change
    mut keyboard_state: ResMut<KeyboardState>,
) {
    for ev in double_click_evr.read() {
        debug!("{:?}", ev);
        match ev.btn {
            MouseButton::Left => {
                if keyboard_state.target != Entity::PLACEHOLDER {
                    if let Some(s) = keyboard_state.input_buf.as_ref() {
                        cmds.trigger_targets(
                            TextRefreshEvent::Finish(s.to_owned()),
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
                    Transform::from_xyz(ev.world_pos.x, ev.world_pos.y, 0.),
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
                        move |trigger: Trigger<TextRefreshEvent>, mut text: Query<&mut Text2d>| {
                            if let Ok(mut t) = text.get_mut(trigger.entity()) {
                                match trigger.event() {
                                    TextRefreshEvent::Inputing(s) => {
                                        t.0 = format!("{s}|");
                                    }
                                    TextRefreshEvent::Finish(s) => {
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
        }
    }
}
