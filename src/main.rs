mod event;
mod input;
mod setup;

use bevy::prelude::*;
use event::{DebugEventPlugin, EventPlugin};
use input::MouseClickPlugin;
use setup::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MouseClickPlugin)
        .add_plugins(DebugEventPlugin)
        .add_plugins(EventPlugin)
        .add_systems(Startup, setup)
        .run();
}
