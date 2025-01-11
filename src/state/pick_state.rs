use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct PickState {
    pub picked: Vec<Entity>,
}
