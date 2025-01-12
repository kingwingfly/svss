use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct PickState {
    pub active: bool,
    pub picked: Vec<Entity>,
}
