use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct DoubleClickEvent {
    pub btn: MouseButton,
    pub world_pos: Vec2,
    pub window_pos: Vec2,
}
