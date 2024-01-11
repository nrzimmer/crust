use std::io;

use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, Stylize};
use crossterm::QueueableCommand;

use crate::tui::constants::MIN_BUFFER_LIST_WIDTH;
use crate::tui::position::{Point, Size};
use crate::tui::traits::Draw;
use crate::{impl_dirty, impl_resize};

pub struct BufferList {
    pub pos: Point,
    pub size: Size,
    list: Vec<String>,
    dirty: bool,
}

impl BufferList {
    pub fn new(_width: u16, height: u16) -> Self {
        BufferList {
            pos: (0, 0).into(),
            size: (MIN_BUFFER_LIST_WIDTH, height).into(),
            list: Vec::new(),
            dirty: true,
        }
    }
}

impl Draw for BufferList {
    fn draw(&mut self, out: &mut impl QueueableCommand) -> io::Result<()> {
        if self.dirty {
            self.dirty = false;
            for (i, text) in self.list.iter().enumerate() {
                out.queue(MoveTo(self.pos.x, self.pos.y + i as u16))?;
                out.queue(Print(text.clone().with(Color::Red)))?;
            }
        }
        Ok(())
    }
}

impl_resize!(for BufferList);
impl_dirty!(for BufferList);
