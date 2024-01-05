// The initial version of this file is "heavily based on"/"copied from" Tsoding 4at chat client.
// Project: https://github.com/tsoding/4at
// File: https://github.com/tsoding/4at/blob/789976acf2764bd4733b05f7d06f2cc889c1cc4c/src/client.rs
//
// For more information:
//
// Youtube playlist about the project: https://www.youtube.com/watch?v=qmmQAAJzM54&list=PLpM-Dvs8t0VZ1tPn-Qqdro3p_5s1HuMyF
// Twitch: https://www.twitch.tv/tsoding

use std::{io, mem};
use std::io::Write;

use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::style::{Attributes, Color, Print, SetAttributes, SetBackgroundColor, SetForegroundColor, SetUnderlineColor};
use crossterm::terminal::{Clear, ClearType};
use crate::tui::AT_RST;

pub struct DoubleBuffer {
    curr: Vec<Cell>,
    prev: Vec<Cell>,
    width: usize,
    height: usize,
}

impl DoubleBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            curr: vec![Cell::default(); width * height],
            prev: vec![Cell::default(); width * height],
            width,
            height,
        }
    }

    pub fn swap(&mut self) {
        mem::swap(&mut self.curr, &mut self.prev);
    }

    pub fn resize(&mut self, w: usize, h: usize) {
        if self.width != w || self.height != h {
            self.curr.resize(w * h, Cell::default());
            self.prev.resize(w * h, Cell::default());
            self.curr.fill(Cell::default());
            self.prev.fill(Cell::default());
            self.width = w;
            self.height = h;
        }
    }

    pub fn clear(&mut self) {
        self.curr.fill(Cell::default());
    }

    pub fn put_cell(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Color, ul: Color, at: Attributes) {
        if let Some(cell) = self.curr.get_mut(y * self.width + x) {
            *cell = Cell { ch, fg, bg, ul, at }
        }
    }

    pub fn put_cells(&mut self, x: usize, y: usize, chs: &[char], fg: Color, bg: Color, ul: Color, at: Attributes) {
        let start = y * self.width + x;
        for (offset, &ch) in chs.iter().enumerate() {
            if let Some(cell) = self.curr.get_mut(start + offset) {
                *cell = Cell { ch, fg, bg, ul, at };
            } else {
                break;
            }
        }
    }

    pub fn flush(&self, qc: &mut impl Write) -> io::Result<()> {
        let mut fg_curr = Color::White;
        let mut bg_curr = Color::Black;
        let mut ul_curr = Color::Reset;
        let mut at_curr: Attributes = AT_RST.clone();
        qc.queue(Clear(ClearType::All))?;
        qc.queue(SetForegroundColor(fg_curr))?;
        qc.queue(SetBackgroundColor(bg_curr))?;
        qc.queue(SetUnderlineColor(bg_curr))?;
        qc.queue(SetAttributes(at_curr))?;
        qc.queue(MoveTo(0, 0))?;
        for &Cell { ch, fg, bg , ul, at} in self.prev.iter() {
            if fg_curr != fg {
                fg_curr = fg;
                qc.queue(SetForegroundColor(fg_curr))?;
            }
            if bg_curr != bg {
                bg_curr = bg;
                qc.queue(SetBackgroundColor(bg_curr))?;
            }
            if ul_curr != ul {
                ul_curr = ul;
                qc.queue(SetUnderlineColor(ul_curr))?;
            }
            if at_curr != at {
                at_curr = at;
                qc.queue(SetAttributes(at_curr))?;
            }
            qc.queue(Print(ch))?;
        }
        qc.flush()?;
        Ok(())
    }

    pub fn update(&mut self, qc: &mut impl QueueableCommand) -> io::Result<()> {
        let mut fg_curr = Color::White;
        let mut bg_curr = Color::Black;
        let mut ul_curr = Color::Reset;
        let mut at_curr = AT_RST.clone();
        let mut x_prev = 0;
        let mut y_prev = 0;
        qc.queue(SetForegroundColor(fg_curr))?;
        qc.queue(SetBackgroundColor(bg_curr))?;
        qc.queue(SetUnderlineColor(bg_curr))?;
        qc.queue(SetAttributes(at_curr))?;

        self.prev.iter().zip(self.curr.iter()).enumerate().filter(|(_, (a, b))| {
            *a != *b
        }).try_for_each(
            |(i, (_, &Cell { ch, fg, bg , ul, at}))| {
                let x = i % self.width;
                let y = i / self.width;

                if y_prev != y || x_prev + 1 != x {
                    qc.queue(MoveTo(x as u16, y as u16))?;
                }

                x_prev = x;
                y_prev = y;

                if fg_curr != fg {
                    fg_curr = fg;
                    qc.queue(SetForegroundColor(fg_curr))?;
                }

                if bg_curr != bg {
                    bg_curr = bg;
                    qc.queue(SetBackgroundColor(bg_curr))?;
                }

                if ul_curr != ul {
                    ul_curr = ul;
                    qc.queue(SetUnderlineColor(ul_curr))?;
                }

                if at_curr != at {
                    at_curr = at;
                    qc.queue(SetAttributes(at_curr))?;
                }

                qc.queue(Print(ch))?;

                Ok(())
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
    ul: Color,
    at: Attributes,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::White,
            bg: Color::Black,
            ul: Color::Reset,
            at: AT_RST.clone(),
        }
    }
}