use crate::tui::constants::MIN_NICK_LIST_WIDTH;
use crate::tui::position::{Point, Size};
use crate::tui::traits::Draw;
use crate::{impl_dirty, impl_resize};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, Stylize};
use crossterm::QueueableCommand;
use std::io;

pub struct NickList {
    pub pos: Point,
    pub size: Size,
    list: Vec<String>,
    dirty: bool,
}

impl NickList {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            pos: (width - MIN_NICK_LIST_WIDTH, 1).into(),
            size: (MIN_NICK_LIST_WIDTH, height - 3).into(),
            list: Vec::new(),
            dirty: true,
        }
    }
}

impl Draw for NickList {
    fn draw(&mut self, out: &mut impl QueueableCommand) -> io::Result<()> {
        if self.dirty {
            self.dirty = false;
            for (i, text) in self.list.iter().enumerate() {
                out.queue(MoveTo(self.pos.x, self.pos.y + i as u16))?;
                out.queue(Print(text.clone().with(Color::Cyan)))?;
            }
        }
        Ok(())
    }
}

impl_resize!(for NickList);
impl_dirty!(for NickList);
