use bevy::prelude::*;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MouseClickEvent>();
    }
}

pub struct DebugEventPlugin;

impl Plugin for DebugEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_event);
    }
}

#[derive(Debug, Event)]
pub enum MouseClickEvent {
    SingleClick(MouseButton),
    DoubleClick(MouseButton),
    None,
}

fn debug_event(mut ev_r: EventReader<MouseClickEvent>) {
    for ev in ev_r.read() {
        println!("{:?}", ev);
    }
}
