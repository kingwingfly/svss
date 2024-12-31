use bevy::prelude::*;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MouseClickEvent>()
            .add_systems(Update, event);
    }
}

#[derive(Debug, Event)]
pub enum MouseClickEvent {
    SingleClick(MouseButton),
    DoubleClick(MouseButton),
    None,
}

fn event(mut ev_r: EventReader<MouseClickEvent>) {
    for ev in ev_r.read() {
        debug!("{:?}", ev);
    }
}
