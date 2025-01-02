use bevy::prelude::*;

const DOUBLE_CLICK_THRESHOLD: f32 = 0.25; // in secs

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
pub struct DoubleClickState {
    timer: Timer,
    last_btn: Option<MouseButton>,
}

impl Default for DoubleClickState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(DOUBLE_CLICK_THRESHOLD, TimerMode::Once),
            last_btn: None,
        }
    }
}

impl DoubleClickState {
    pub fn tick(&mut self, duration: std::time::Duration) {
        self.timer.tick(duration);
    }

    pub fn click(&mut self, btn: Option<MouseButton>) -> Option<MouseButton> {
        match self.last_btn {
            // no btn recorded
            None => {
                if btn.is_some() {
                    self.timer.reset();
                    self.last_btn = btn;
                }
                None
            }
            // timer finished, sigle click
            Some(_) if self.timer.just_finished() => {
                self.timer.reset();
                self.last_btn = btn;
                None
            }
            // timer not finished, double click
            Some(last_btn) if btn == Some(last_btn) => {
                self.last_btn = None;
                Some(last_btn)
            }
            // timer not finished, but different btn
            Some(last_btn) if btn.is_some() && btn != Some(last_btn) => {
                self.timer.reset();
                self.last_btn = btn;
                None
            }
            _ => None,
        }
    }
}
