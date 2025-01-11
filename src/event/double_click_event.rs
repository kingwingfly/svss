use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct CreateNodeEvent {
    pub world_pos: Vec2,
}

#[derive(Debug, Event)]
pub struct EditEvent;
