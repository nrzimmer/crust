use crate::tui::constants::*;
use crate::tui::position::{Point, Size};
use crate::tui::traits::Draw;
use crate::{impl_dirty, impl_resize};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, Stylize};
use crossterm::QueueableCommand;
use std::io;

pub struct VertBar {
    pub pos: Point,
    pub size: Size,
    ch: char,
    dirty: bool,
}

pub enum VertBarType {
    Left,
    Right,
}

impl VertBar {
    pub fn new(width: u16, height: u16, vert_bar_type: VertBarType) -> Self {
        match vert_bar_type {
            VertBarType::Left => Self {
                pos: (MIN_BUFFER_LIST_WIDTH, 0).into(),
                size: (1, height).into(),
                ch: '│',
                dirty: true,
            },
            VertBarType::Right => Self {
                pos: (width - MIN_NICK_LIST_WIDTH - 1, 1).into(),
                size: (1, height - 3).into(),
                ch: '│',
                dirty: true,
            },
        }
    }
}

impl Draw for VertBar {
    fn draw(&mut self, out: &mut impl QueueableCommand) -> io::Result<()> {
        if self.dirty {
            self.dirty = false;
            for i in 0..self.size.height {
                out.queue(MoveTo(self.pos.x, self.pos.y + i))?;
                out.queue(Print(self.ch.with(Color::Blue)))?;
            }
        }
        Ok(())
    }
}

impl_resize!(for VertBar);
impl_dirty!(for VertBar);
