use bevy::prelude::*;

pub struct MouseClickPlugin;

impl Plugin for MouseClickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseClickState::default())
            .add_systems(Update, mouse_click);
    }
}

#[derive(Resource, Debug)]
struct MouseClickState {
    timer: Option<Timer>,
    last_btn: Option<MouseButton>,
    double_click_threshold: f32,
}

impl Default for MouseClickState {
    fn default() -> Self {
        Self {
            timer: None,
            last_btn: None,
            double_click_threshold: 0.3,
        }
    }
}

#[derive(Debug)]
enum MouseClickEvent {
    SingleClick(MouseButton),
    DoubleClick(MouseButton),
    None,
}

impl MouseClickState {
    fn tick(&mut self, duration: std::time::Duration) {
        if let Some(timer) = self.timer.as_mut() {
            timer.tick(duration);
        }
    }

    fn click(&mut self, btn: Option<MouseButton>) -> MouseClickEvent {
        match (self.timer.as_mut(), self.last_btn) {
            (_, None) => {
                if btn.is_some() {
                    self.timer = Some(Timer::from_seconds(
                        self.double_click_threshold,
                        TimerMode::Once,
                    ));
                    self.last_btn = btn;
                }
                MouseClickEvent::None
            }
            (Some(timer), Some(last_btn)) if timer.just_finished() => {
                timer.reset();
                self.last_btn = btn;
                MouseClickEvent::SingleClick(last_btn)
            }
            (ref mut timer, Some(last_btn)) if btn == Some(last_btn) => {
                *timer = None;
                self.last_btn = None;
                MouseClickEvent::DoubleClick(last_btn)
            }
            (Some(timer), Some(last_btn)) if btn.is_some() && btn != Some(last_btn) => {
                timer.reset();
                self.last_btn = btn;
                MouseClickEvent::SingleClick(last_btn)
            }
            _ => MouseClickEvent::None,
        }
    }
}

fn mouse_click(
    time: Res<Time>,
    mouse_input_events: Res<ButtonInput<MouseButton>>,
    mut click_state: ResMut<MouseClickState>,
) {
    click_state.tick(time.delta());
    let mut btns = mouse_input_events.get_just_pressed();
    loop {
        match click_state.click(btns.next().cloned()) {
            MouseClickEvent::SingleClick(btn) => {
                println!("Single click: {:?}", btn);
            }
            MouseClickEvent::DoubleClick(btn) => {
                println!("Double click: {:?}", btn);
            }
            MouseClickEvent::None => break,
        }
    }
}