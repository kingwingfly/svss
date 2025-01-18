use super::MyQuadTree;
use crate::{camera::PrimaryCamera, state::PickState};
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
    time: Res<Time>,
    mut gizmos: Gizmos,
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut pick_state: ResMut<PickState>,
    mut start: Local<Option<Vec2>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    quadtree: Res<MyQuadTree>,
    mut cd: Local<PickCD>,
) {
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
                quadtree.query::<_, QOr<(Overlap, Contain)>>(&CollisionRect::from(
                    Rect::from_corners(start, world_pos),
                ))
            } else {
                // right pick
                quadtree
                    .query::<_, Contain>(&CollisionRect::from(Rect::from_corners(start, world_pos)))
            };
            if cancel_pick {
                for e in res {
                    pick_state.picked.remove(&e);
                }
            } else {
                pick_state.picked.extend(res);
            }
            debug!("pick result: {:?}", pick_state.picked);
        }
        None => *start = Some(world_pos),
    }
}
