mod click_input;
mod keyboard_input;

use crate::state::{DoubleClickState, TextInputState};
use bevy::prelude::*;
use click_input::double_click;
use keyboard_input::{ime_input, ime_toggle, text_input};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DoubleClickState>()
            .init_resource::<TextInputState>()
            .add_systems(Update, (double_click, text_input, ime_toggle, ime_input));
    }
}
