use bevy::prelude::*;
use core::fmt;

const DOUBLE_CLICK_THRESHOLD: f32 = 0.25; // in secs

#[derive(Resource, Debug)]
pub struct KeyboardState {
    input_buf: Vec<String>,
    cursor: (usize, usize),
    pub target: Entity,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            input_buf: vec![String::new()],
            cursor: (0, 0),
            target: Entity::PLACEHOLDER,
        }
    }
}

impl KeyboardState {
    pub fn reset(&mut self) -> String {
        let res = self.input_buf.join("\n");
        *self = Self::default();
        res
    }

    pub fn move_left(&mut self) {
        if self.cursor.1 == 0 {
            if self.cursor.0 == 0 {
                return;
            }
            self.cursor.0 -= 1;
            self.cursor.1 = self.input_buf[self.cursor.0].len();
        } else {
            self.cursor.1 = self.cursor.1.saturating_sub(1);
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor.1 == self.input_buf[self.cursor.0].len() {
            if self.cursor.0 == self.input_buf.len() - 1 {
                return;
            }
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        } else {
            self.cursor.1 = self
                .cursor
                .1
                .saturating_add(1)
                .min(self.input_buf[self.cursor.0].len());
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.0 == 0 {
            return;
        }
        self.cursor.0 -= 1;
        self.cursor.1 = self.cursor.1.min(self.input_buf[self.cursor.0].len());
    }

    pub fn move_down(&mut self) {
        if self.cursor.0 == self.input_buf.len() - 1 {
            return;
        }
        self.cursor.0 += 1;
        self.cursor.1 = self.cursor.1.min(self.input_buf[self.cursor.0].len());
    }

    pub fn backspace(&mut self) {
        if self.cursor.1 == 0 {
            if self.cursor.0 == 0 {
                return;
            }
            let line = self.input_buf.remove(self.cursor.0);
            self.cursor.0 -= 1;
            self.cursor.1 = self.input_buf[self.cursor.0].len();
            self.input_buf[self.cursor.0].push_str(&line);
        } else {
            self.cursor.1 -= 1;
            self.input_buf[self.cursor.0].remove(self.cursor.1);
        }
    }

    pub fn insert_str(&mut self, s: &str) {
        self.input_buf[self.cursor.0].insert_str(self.cursor.1, s);
        self.cursor.1 += s.len();
    }

    pub fn new_line(&mut self) {
        let new_line = self.input_buf[self.cursor.0]
            .drain(self.cursor.1..)
            .collect();
        self.input_buf.insert(self.cursor.0 + 1, new_line);
        self.cursor.0 += 1;
        self.cursor.1 = 0;
    }
}

impl fmt::Display for KeyboardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, line) in self.input_buf.iter().enumerate() {
            if i == self.cursor.0 {
                write!(f, "{}|{}", &line[..self.cursor.1], &line[self.cursor.1..])?;
            } else {
                write!(f, "{}", line)?;
            }
            if i != self.input_buf.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
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
