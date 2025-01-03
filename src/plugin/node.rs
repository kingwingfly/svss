use crate::{
    event::{DoubleClickEvent, TextRefreshEvent},
    state::TextInputState,
};
use bevy::{color::palettes::css::*, prelude::*};

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
    // double click leads text input target change
    mut keyboard_state: ResMut<TextInputState>,
) {
    for ev in double_click_evr.read() {
        debug!("{:?}", ev);
        match ev.btn {
            MouseButton::Left => {
                if keyboard_state.target != Entity::PLACEHOLDER {
                    let target = keyboard_state.target;
                    cmds.trigger_targets(TextRefreshEvent(keyboard_state.reset()), target);
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
                    keyboard_state.target = entity_cmds.id();
                    entity_cmds.observe(
                        move |trigger: Trigger<TextRefreshEvent>, mut text: Query<&mut Text2d>| {
                            if let Ok(mut t) = text.get_mut(trigger.entity()) {
                                t.0 = trigger.event().0.clone();
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
