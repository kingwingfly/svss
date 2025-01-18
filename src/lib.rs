#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod camera;
mod event;
mod graph;
mod input;
mod setup;
mod state;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use camera::MyCameraPlugin;
use graph::GraphPlugin;
use input::InputPlugin;
use setup::setup;

pub fn run() {
    let mut app = App::new();
    #[cfg(debug_assertions)]
    {
        app.add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            ..Default::default()
        }));
    }
    #[cfg(not(debug_assertions))]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: Level::INFO,
        ..Default::default()
    }));
    app.add_plugins((InputPlugin, GraphPlugin, MyCameraPlugin))
        .add_systems(Startup, setup)
        .run();
}
