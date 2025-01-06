use bevy::prelude::*;

use crate::state::TextInputState;

#[derive(Debug, Clone, Component)]
pub struct TextRefreshEvent {
    pub text: String,
    pub height: f32,
    pub width: f32,
}

impl Event for TextRefreshEvent {
    type Traversal = &'static Parent;
    const AUTO_PROPAGATE: bool = true;
}

impl From<&TextInputState> for TextRefreshEvent {
    fn from(state: &TextInputState) -> Self {
        Self {
            text: state.to_string(),
            height: state.height() as f32,
            width: state.width() as f32,
        }
    }
}
