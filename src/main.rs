#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod event;
mod input;
mod setup;
mod state;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use event::EventPlugin;
use input::InputPlugin;
use setup::setup;

fn main() {
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
    app.add_plugins(InputPlugin)
        .add_plugins(EventPlugin)
        .add_systems(Startup, setup)
        .run();
}
