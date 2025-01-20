use super::MyQuadTree;
use crate::{
    camera::PrimaryCamera,
    state::{PickState, TextInputState},
};
use bevy::{color::palettes::css::*, prelude::*};
use bevy_quadtree::{CollisionRect, Contain, Overlap, QOr};

#[derive(Debug, Component, Deref, DerefMut)]
pub struct PickCD(Timer);

impl Default for PickCD {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

pub fn pick(
    mut cmds: Commands,
    time: Res<Time>,
    mut gizmos: Gizmos,
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut pick_state: ResMut<PickState>,
    input_state: Res<TextInputState>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    quadtree: Res<MyQuadTree>,
    mut start: Local<Option<Vec2>>,
    mut cd: Local<PickCD>,
) {
    if key.just_pressed(KeyCode::Escape) || input_state.target != Entity::PLACEHOLDER {
        *start = None;
        for e in pick_state.picked.drain() {
            cmds.entity(e)
                .entry::<Sprite>()
                .and_modify(|mut s| s.color = Color::WHITE);
        }
        return;
    }
    if !btn.pressed(MouseButton::Left) || !pick_state.active {
        *start = None;
        return;
    }
    let window = q_window.single();
    let Some(pos) = window.cursor_position() else {
        return;
    };
    let (camera, camera_transform) = q_camera.single();
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) else {
        return;
    };
    let cancel_pick = key.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    match *start {
        Some(start) => {
            gizmos.rect_2d(
                (start + world_pos) / 2.,
                (start - world_pos).abs(),
                if cancel_pick { RED } else { WHITE },
            );
            if !cd.tick(time.delta()).just_finished() {
                return;
            }
            cd.reset();
            let res = if start.x > world_pos.x {
                // left pick
                quadtree.query::<QOr<(Overlap, Contain)>>(&CollisionRect::from(Rect::from_corners(
                    start, world_pos,
                )))
            } else {
                // right pick
                quadtree
                    .query::<Contain>(&CollisionRect::from(Rect::from_corners(start, world_pos)))
            };
            if cancel_pick {
                for e in res {
                    if pick_state.picked.remove(&e) {
                        cmds.entity(e)
                            .entry::<Sprite>()
                            .and_modify(|mut s| s.color = Color::WHITE);
                    }
                }
            } else {
                for e in res {
                    if pick_state.picked.insert(e) {
                        cmds.entity(e)
                            .entry::<Sprite>()
                            .and_modify(|mut s| s.color = GREEN.into());
                    }
                }
            }
        }
        None => *start = Some(world_pos),
    }
}

pub fn delete_picked(
    mut cmds: Commands,
    mut pick_state: ResMut<PickState>,
    key: Res<ButtonInput<KeyCode>>,
    input_state: Res<TextInputState>,
) {
    if input_state.target != Entity::PLACEHOLDER {
        return;
    }
    if key.any_just_pressed([KeyCode::Backspace, KeyCode::Delete]) {
        for e in pick_state.picked.drain() {
            cmds.entity(e).despawn_recursive();
        }
    }
}
