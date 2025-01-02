use bevy::prelude::*;

#[derive(Debug, Event)]
pub enum TextRefreshEvent {
    Inputing(String),
    Finish(String),
}
