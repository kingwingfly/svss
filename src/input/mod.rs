mod keyboard_input;
mod pick_input;

use crate::state::{PickState, TextInputState};
use bevy::prelude::*;
use keyboard_input::{ime_input, ime_toggle, text_input};
use pick_input::{left_pick, right_pick};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextInputState>()
            .init_resource::<PickState>()
            .add_systems(Update, (text_input, ime_toggle, ime_input))
            .add_systems(Update, (left_pick, right_pick));
    }
}
