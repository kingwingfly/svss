use bevy::prelude::*;
use std::time::{Duration, Instant};

/// Instant wrapper for observer system,
/// since `Timer.tick(Time.delta())` cannot always be called after `Time::update()` in oberver system,
/// which leads timer recording wrong delta.
#[derive(Resource, Deref, DerefMut)]
pub struct MyTime(Instant);

impl Default for MyTime {
    fn default() -> Self {
        MyTime(Instant::now())
    }
}

impl MyTime {
    pub fn update(&mut self) -> Duration {
        let now = Instant::now();
        let res = now.duration_since(**self);
        **self = now;
        res
    }
}
