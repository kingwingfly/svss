use crate::state::AppState;
use bevy::prelude::*;

pub fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
    cmds.spawn(AppState::new());
}
