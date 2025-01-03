mod click_input;
mod keyboard_input;

use crate::state::{DoubleClickState, KeyboardState};
use bevy::prelude::*;
use click_input::double_click;
use keyboard_input::text_input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DoubleClickState>()
            .init_resource::<KeyboardState>()
            .add_systems(Update, (double_click, text_input));
    }
}
