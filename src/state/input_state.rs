use bevy::prelude::*;
use core::fmt;
use std::time::Instant;

const DOUBLE_CLICK_THRESHOLD: f32 = 0.25; // in secs, should <= 0.25

#[derive(Resource, Debug)]
pub struct TextInputState {
    pub input_buf: Vec<Vec<char>>,
    pub ime_buf: String,
    ime_buf_cursor: (usize, usize),
    cursor: (usize, usize),
    submitted: bool,
    /// target entity to receive the text
    pub target: Entity,
}

impl Default for TextInputState {
    fn default() -> Self {
        Self {
            input_buf: vec![vec![]],
            ime_buf: String::new(),
            ime_buf_cursor: (0, 0),
            cursor: (0, 0),
            submitted: false,
            target: Entity::PLACEHOLDER,
        }
    }
}

impl TextInputState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// influence the rendering of the text
    pub fn submit(&mut self) {
        self.submitted = true;
    }

    pub fn height(&self) -> usize {
        self.input_buf.len()
    }

    pub fn width(&self) -> usize {
        self.input_buf
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let mut width = 0.;
                for c in s {
                    if c.is_ascii() {
                        width += 0.5;
                    } else {
                        width += 1.;
                    }
                }
                if i == self.cursor.0 {
                    width += self.ime_buf_cursor.1 as f32;
                    if !self.submitted {
                        width += 0.5; // the cursor
                    }
                }
                width.ceil() as usize
            })
            .max()
            .unwrap_or(0)
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

    pub fn move_to_line_head(&mut self) {
        self.cursor.1 = 0;
    }

    pub fn move_to_line_tail(&mut self) {
        self.cursor.1 = self.input_buf[self.cursor.0].len();
    }

    pub fn move_to_head(&mut self) {
        self.cursor = (0, 0);
    }

    pub fn move_to_tail(&mut self) {
        self.cursor = (
            self.input_buf.len() - 1,
            self.input_buf.last().unwrap().len(),
        );
    }

    pub fn backspace(&mut self) {
        if !self.ime_buf.is_empty() {
            if self.ime_buf.len() == 1 {
                self.ime_buf.pop();
                self.ime_buf_cursor = (0, 0);
            }
            return;
        }
        if self.cursor.1 == 0 {
            if self.cursor.0 == 0 {
                return;
            }
            let line = self.input_buf.remove(self.cursor.0);
            self.cursor.0 -= 1;
            self.cursor.1 = self.input_buf[self.cursor.0].len();
            self.input_buf[self.cursor.0].extend_from_slice(&line);
        } else {
            self.cursor.1 -= 1;
            self.input_buf[self.cursor.0].remove(self.cursor.1);
        }
    }

    pub fn insert_str(&mut self, s: &str) {
        for c in s.chars() {
            self.input_buf[self.cursor.0].insert(self.cursor.1, c);
            self.cursor.1 += 1;
        }
        self.ime_buf_cursor = (0, 0);
        self.ime_buf.clear();
    }

    pub fn new_line(&mut self) {
        let new_line = self.input_buf[self.cursor.0]
            .drain(self.cursor.1..)
            .collect();
        self.input_buf.insert(self.cursor.0 + 1, new_line);
        self.cursor.0 += 1;
        self.cursor.1 = 0;
    }

    pub fn set_ime_buf(&mut self, s: &str, cursor: (usize, usize)) {
        self.ime_buf = s.to_string();
        self.ime_buf_cursor = cursor;
    }
}

impl fmt::Display for TextInputState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.submitted {
            true => {
                for (i, line) in self.input_buf.iter().enumerate() {
                    write!(f, "{}", line.iter().collect::<String>())?;
                    if i != self.input_buf.len() - 1 {
                        writeln!(f)?;
                    }
                }
            }
            false => {
                for (i, line) in self.input_buf.iter().enumerate() {
                    if i == self.cursor.0 {
                        write!(
                            f,
                            "{}{}|{}{}",
                            line[..self.cursor.1].iter().collect::<String>(),
                            &self.ime_buf[..self.ime_buf_cursor.0],
                            &self.ime_buf[self.ime_buf_cursor.0..],
                            line[self.cursor.1..].iter().collect::<String>()
                        )?;
                    } else {
                        write!(f, "{}", line.iter().collect::<String>())?;
                    }
                    if i != self.input_buf.len() - 1 {
                        writeln!(f)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Resource, Debug)]
pub struct DoubleClickState {
    time: Instant,
    timer: Timer,
    last_btn: Option<PointerButton>,
}

impl Default for DoubleClickState {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            timer: Timer::from_seconds(DOUBLE_CLICK_THRESHOLD, TimerMode::Once),
            last_btn: None,
        }
    }
}

impl DoubleClickState {
    pub fn click(&mut self, btn: Option<PointerButton>) -> Option<PointerButton> {
        let now = Instant::now();
        self.timer.tick(now.duration_since(self.time));
        self.time = now;
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
