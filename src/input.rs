use bevy::prelude::*;
use std::time::Instant;

pub struct MouseClickPlugin;

impl Plugin for MouseClickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_double_click);
    }
}

#[derive(Resource)]
struct MouseClickState {
    last_click_time: Option<Instant>,
    last_click_btn: Option<MouseButton>,
    double_click_threshold: std::time::Duration,
}

impl Default for MouseClickState {
    fn default() -> Self {
        Self {
            last_click_time: None,
            last_click_btn: None,
            double_click_threshold: std::time::Duration::from_millis(500),
        }
    }
}

fn mouse_double_click(
    mouse_input_events: Res<ButtonInput<MouseButton>>,
    mut click_state: Local<MouseClickState>,
) {
    for btn in mouse_input_events.get_just_pressed() {
        let now = Instant::now();
        match (
            click_state.last_click_time.replace(now),
            click_state.last_click_btn.replace(*btn),
        ) {
            (Some(last_time), Some(last_btn))
                if last_btn == *btn && now - last_time < click_state.double_click_threshold =>
            {
                println!("double click: {:?}", btn);
            }
            _ => {}
        }
    }
}
