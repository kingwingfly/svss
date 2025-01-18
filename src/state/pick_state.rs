use bevy::{ecs::entity::EntityHashSet, prelude::*};

#[derive(Resource, Default, Debug)]
pub struct PickState {
    pub active: bool,
    pub picked: EntityHashSet,
}
