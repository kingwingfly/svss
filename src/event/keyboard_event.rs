use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct TextRefreshEvent(pub String);
