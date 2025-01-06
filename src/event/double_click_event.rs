use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct DoubleClickEvent {
    pub btn: MouseButton,
    pub world_pos: Vec2,
}
