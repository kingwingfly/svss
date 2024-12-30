mod input;
mod setup;

use bevy::prelude::*;
use input::MouseClickPlugin;
use setup::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MouseClickPlugin)
        .add_systems(Startup, setup)
        .run();
}
