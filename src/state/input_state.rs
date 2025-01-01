use bevy::prelude::*;

use crate::event::ClickEvent;

#[derive(Resource, Debug)]
pub struct KeyboardState {
    pub input_buf: Option<String>,
    pub target: Entity,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            input_buf: None,
            target: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Resource, Debug)]
pub struct ClickState {
    timer: Option<Timer>,
    last_btn: Option<MouseButton>,
    double_click_threshold: f32,
}

impl Default for ClickState {
    fn default() -> Self {
        Self {
            timer: None,
            last_btn: None,
            double_click_threshold: 0.25,
        }
    }
}

impl ClickState {
    pub fn tick(&mut self, duration: std::time::Duration) {
        if let Some(timer) = self.timer.as_mut() {
            timer.tick(duration);
        }
    }

    pub fn click(&mut self, btn: Option<MouseButton>) -> ClickEvent {
        match (self.timer.as_mut(), self.last_btn) {
            // no btn recorded
            (_, None) => {
                if btn.is_some() {
                    self.timer = Some(Timer::from_seconds(
                        self.double_click_threshold,
                        TimerMode::Once,
                    ));
                    self.last_btn = btn;
                }
                ClickEvent::None
            }
            // timer finished, sigle click
            (Some(timer), Some(last_btn)) if timer.just_finished() => {
                self.timer = None;
                self.last_btn = btn;
                ClickEvent::SingleClick(last_btn)
            }
            // timer not finished, double click
            (Some(_), Some(last_btn)) if btn == Some(last_btn) => {
                self.timer = None;
                self.last_btn = None;
                ClickEvent::DoubleClick(last_btn)
            }
            // timer not finished, but different btn
            (Some(timer), Some(last_btn)) if btn.is_some() && btn != Some(last_btn) => {
                timer.reset();
                self.last_btn = btn;
                ClickEvent::SingleClick(last_btn)
            }
            _ => ClickEvent::None,
        }
    }
}
