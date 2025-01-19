mod background;
mod edge;
mod node;

use crate::event::{CreateNodeEvent, EditEvent};
use background::BackgroundPlugin;
use bevy::app::Plugin;
use node::NodePlugin;

pub use background::SIZE;
pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CreateNodeEvent>()
            .add_event::<EditEvent>()
            .add_plugins((NodePlugin, BackgroundPlugin));
    }
}
