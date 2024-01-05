// The initial version of this file is "heavily based on"/"copied from" Tsoding 4at chat client.
// Project: https://github.com/tsoding/4at
// File: https://github.com/tsoding/4at/blob/789976acf2764bd4733b05f7d06f2cc889c1cc4c/src/client.rs
//
// For more information:
//
// Youtube playlist about the project: https://www.youtube.com/watch?v=qmmQAAJzM54&list=PLpM-Dvs8t0VZ1tPn-Qqdro3p_5s1HuMyF
// Twitch: https://www.twitch.tv/tsoding

use std::{cmp, io};
use std::io::Write;

use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::style::{Attributes, Color};
use crate::tui::AT_RST;

use crate::tui::doublebuffer::DoubleBuffer;

#[derive(Default)]
pub struct Prompt {
    pub buffer: Vec<char>,
    cursor: usize,
    scroll: usize,
}

impl Prompt {
    fn sync_scroll_with_cursor(&mut self, w: usize) {
        if self.cursor < self.scroll {
            self.scroll = self.cursor;
        }
        if self.scroll + w <= self.cursor {
            self.scroll = self.cursor - w;
        }
    }

    pub fn sync_terminal_cursor(
        &mut self,
        qc: &mut impl Write,
        x: usize,
        y: usize,
        w: usize,
    ) -> io::Result<()> {
        if let Some(w) = w.checked_sub(2) {
            let x = x + 1;
            self.sync_scroll_with_cursor(w);
            let offset = self.cursor - self.scroll; // NOTE: self.scroll <= self.cursor must be guaranteed by self.sync_scroll_with_cursor()
            let _ = qc.queue(MoveTo((x + offset) as u16, y as u16))?;
        }
        Ok(())
    }

    pub fn render(&mut self, buffer: &mut DoubleBuffer, x: usize, y: usize, w: usize) {
        if let Some(w) = w.checked_sub(2) {
            let x = x + 1;
            self.sync_scroll_with_cursor(w);
            let begin = self.scroll;
            let end = cmp::min(self.scroll + w, self.buffer.len());
            if let Some(window) = self.buffer.get(begin..end) {
                buffer.put_cells(x, y, window, Color::White, Color::Black, Color::Reset, AT_RST.clone());
                if self.scroll > 0 {
                    buffer.put_cell(x - 1, y, '<', Color::White, Color::Black, Color::Reset, AT_RST.clone());
                }
                if self.scroll + w < self.buffer.len() {
                    buffer.put_cell(x + w, y, '>', Color::White, Color::Black, Color::Reset, AT_RST.clone());
                }
            }
        }
    }

    pub fn insert(&mut self, x: char) {
        if self.cursor > self.buffer.len() {
            self.cursor = self.buffer.len()
        }
        self.buffer.insert(self.cursor, x);
        self.cursor += 1;
    }

    pub fn insert_str(&mut self, text: &str) {
        for x in text.chars() {
            self.insert(x)
        }
    }

    pub fn left_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn right_char(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    fn at_cursor(&self) -> char {
        self.buffer.get(self.cursor).cloned().unwrap_or('\n')
    }

    pub fn remove_left_word(&mut self) {
        if self.cursor == self.buffer.len() {
            self.cursor -= 1;
        }
        while self.cursor > 0 && self.at_cursor().is_whitespace() {
            self.buffer.remove(self.cursor);
            self.cursor -= 1;
        }
        while self.cursor > 0 && !self.at_cursor().is_whitespace() {
            self.buffer.remove(self.cursor);
            self.cursor -= 1;
        }
        if self.cursor == 0 && self.buffer.len() > 0 {
            self.buffer.remove(self.cursor);
        }
    }

    pub fn remove_right_word(&mut self) {
        while self.cursor < self.buffer.len() && self.at_cursor().is_whitespace() {
            self.buffer.remove(self.cursor);
        }
        while self.cursor < self.buffer.len() && !self.at_cursor().is_whitespace() {
            self.buffer.remove(self.cursor);
        }
    }

    pub fn left_word(&mut self) {
        while self.cursor > 0 && self.at_cursor().is_whitespace() {
            self.cursor -= 1;
        }
        while self.cursor > 0 && !self.at_cursor().is_whitespace() {
            self.cursor -= 1;
        }
    }

    pub fn right_word(&mut self) {
        while self.cursor < self.buffer.len() && self.at_cursor().is_whitespace() {
            self.cursor += 1;
        }
        while self.cursor < self.buffer.len() && !self.at_cursor().is_whitespace() {
            self.cursor += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.buffer.remove(self.cursor);
        }
    }

    pub fn delete(&mut self) {
        if self.cursor < self.buffer.len() {
            self.buffer.remove(self.cursor);
        }
    }

    pub fn home(&mut self) {
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.cursor = self.buffer.len();
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }

    pub fn delete_until_end(&mut self) {
        while self.cursor < self.buffer.len() {
            self.buffer.pop();
        }
    }
}
