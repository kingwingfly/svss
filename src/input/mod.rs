mod keyboard_input;
mod pick_input;

use crate::state::{PickState, TextInputState};
use bevy::prelude::*;
use bevy_quadtree::{CollisionRect, QuadTree, QuadTreePlugin};
use keyboard_input::{ime_input, ime_toggle, text_input};
use pick_input::{delete_picked, pick};

pub(super) type MyQuadTree = QuadTree<2, 1000, 1000, 20>;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuadTreePlugin::<CollisionRect, 2, 1000, 1000, 20>::default())
            .init_resource::<TextInputState>()
            .init_resource::<PickState>()
            .add_systems(Update, (text_input, ime_toggle, ime_input))
            .add_systems(Update, (pick, delete_picked));
    }
}
