use std::cmp::max;
use std::io;
use std::ops::{AddAssign, SubAssign};

use crossterm::cursor::{MoveTo, MoveToColumn};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, Print, Stylize};
use crossterm::QueueableCommand;

use crate::tui::constants::MIN_BUFFER_LIST_WIDTH;
use crate::tui::position::{Point, Size};
use crate::tui::traits::Draw;
use crate::{impl_dirty, impl_resize};

struct Cursor {
    pos: usize,
    dirty: bool,
}

impl AddAssign<usize> for Cursor {
    fn add_assign(&mut self, rhs: usize) {
        self.pos += rhs;
        self.dirty = true;
    }
}

impl SubAssign<usize> for Cursor {
    fn sub_assign(&mut self, rhs: usize) {
        if self.pos > 0 {
            self.pos -= rhs;
        }
        self.dirty = true;
    }
}

impl Cursor {
    fn sync(&mut self, len: usize) {
        if self.pos > len {
            self.pos = len;
            self.dirty = true;
        }
    }

    fn set(&mut self, pos: usize) {
        self.pos = pos;
        self.dirty = true;
    }

    fn should_move(&mut self, a: u16, b: usize) -> Option<u16> {
        if self.dirty {
            self.dirty = false;
            Some((self.pos + a as usize + b + 1) as u16)
        } else {
            None
        }
    }

    fn isize(&self) -> isize {
        self.pos as isize
    }
}

pub struct Prompt {
    pos: Point,
    pub size: Size,
    text: String,
    dirty: bool,
    buffer: Vec<char>,
    prev_buffer_len: usize,
    cursor: Cursor,
    // scroll: u16,
}

impl Prompt {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            pos: (MIN_BUFFER_LIST_WIDTH + 1, height - 1).into(),
            size: (width - MIN_BUFFER_LIST_WIDTH - 1, 1).into(),
            text: "[@_Fulgore_(i)]".into(),
            dirty: true,
            buffer: Vec::with_capacity(1024),
            prev_buffer_len: 0,
            cursor: Cursor { pos: 0, dirty: true },
            // scroll: 0,
        }
    }

    pub fn key_press(&mut self, event: KeyEvent) -> Option<String> {
        match event.code {
            KeyCode::Char(ch) => {
                self.dirty = true;
                self.buffer.insert(self.cursor.pos, ch);
                self.cursor += 1;
            }
            KeyCode::Up => { /* Todo - implement prompt history */ }
            KeyCode::Down => { /* Todo - implement prompt history */ }
            KeyCode::Left => {
                self.key_left(event);
            }
            KeyCode::Right => {
                self.key_right(event);
            }
            KeyCode::Backspace => {
                self.key_backspace();
            }
            KeyCode::Delete => {
                self.key_delete(event);
            }
            KeyCode::Home => {
                self.cursor.set(0);
            }
            KeyCode::End => {
                self.cursor.set(self.buffer.len());
            }
            KeyCode::Enter => {
                let result = self.buffer.iter().collect::<String>();
                self.buffer.clear();
                self.cursor.set(0);
                self.dirty = true;
                return Some(result);
            }
            _ => {}
        }
        None
    }

    fn key_delete(&mut self, event: KeyEvent) {
        fn remove_chars(prompt: &mut Prompt, when: impl Fn(char) -> bool) {
            while prompt.cursor.pos < prompt.buffer.len() && when(prompt.safe_at(prompt.cursor.isize())) {
                prompt.buffer.remove(prompt.cursor.pos);
            }
        }

        if event.modifiers.contains(KeyModifiers::CONTROL) {
            self.dirty = true;
            remove_chars(self, |c| !c.is_whitespace());
            if self.prev_buffer_len == self.buffer.len() {
                remove_chars(self, |c| c.is_whitespace());
            }
            return;
        }

        if self.cursor.pos < self.buffer.len() {
            self.dirty = true;
            self.buffer.remove(self.cursor.pos);
        }
    }

    fn key_backspace(&mut self) {
        // if event.modifiers.contains(KeyModifiers::CONTROL) { DOES NOT WORK
        if self.cursor.pos > 0 {
            self.dirty = true;
            self.cursor -= 1;
            self.buffer.remove(self.cursor.pos);
        }
    }

    fn key_right(&mut self, event: KeyEvent) {
        if event.modifiers.contains(KeyModifiers::CONTROL) {
            while self.cursor.pos < self.buffer.len() && self.safe_at(self.cursor.isize()).is_whitespace() {
                self.cursor += 1;
            }
            while self.cursor.pos < self.buffer.len() && !self.safe_at(self.cursor.isize()).is_whitespace() {
                self.cursor += 1;
            }
        }
        self.cursor += 1;
    }

    fn key_left(&mut self, event: KeyEvent) {
        if event.modifiers.contains(KeyModifiers::CONTROL) {
            while self.cursor.pos > 0 && self.safe_at(self.cursor.isize() - 1).is_whitespace() {
                self.cursor -= 1;
            }
            while self.cursor.pos > 0 && !self.safe_at(self.cursor.isize() - 1).is_whitespace() {
                self.cursor -= 1;
            }
        } else {
            self.cursor -= 1;
        }
    }

    fn safe_at(&self, idx: isize) -> char {
        let idx = max(idx, 0) as usize;
        self.buffer.get(idx).unwrap_or(&' ').clone()
    }
}

impl Draw for Prompt {
    fn draw(&mut self, out: &mut impl QueueableCommand) -> io::Result<()> {
        if self.dirty {
            self.dirty = false;
            self.cursor.dirty = true;
            let str = format!("{} ", self.text).with(Color::DarkGreen);
            out.queue(MoveTo(self.pos.x, self.pos.y))?;
            out.queue(Print(str))?;
            out.queue(Print(self.buffer.iter().collect::<String>()))?;
            if self.buffer.len() != self.prev_buffer_len {
                out.queue(Print(" ".repeat(self.prev_buffer_len)))?;
                self.prev_buffer_len = self.buffer.len();
            }
        }

        self.cursor.sync(self.buffer.len());

        if let Some(col) = self.cursor.should_move(self.pos.x, self.text.len()) {
            out.queue(MoveToColumn(col))?;
        }
        Ok(())
    }
}

impl_resize!(for Prompt);
impl_dirty!(for Prompt);
