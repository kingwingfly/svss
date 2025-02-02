use crate::state::AppState;
use bevy::{prelude::*, winit::WinitSettings};

pub fn setup(mut cmds: Commands) {
    cmds.insert_resource(WinitSettings::desktop_app());
    cmds.init_resource::<AppState>();
}
