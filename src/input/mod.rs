mod keyboard_input;
mod pick_input;

use crate::state::{PickState, TextInputState};
use bevy::prelude::*;
use bevy_quadtree::{CollisionRect, QuadTree, QuadTreePlugin};
use keyboard_input::{ime_input, ime_toggle, text_input};
use pick_input::pick;

pub(super) type MyQuadTree = QuadTree<40, 10000, 10000, 20>;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuadTreePlugin::<CollisionRect, 40, 10000, 10000, 20>::default());
        app.init_resource::<TextInputState>()
            .init_resource::<PickState>()
            .add_systems(Update, (text_input, ime_toggle, ime_input))
            .add_systems(Update, (pick,));
    }
}
