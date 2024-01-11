use crate::tui::constants::MIN_BUFFER_LIST_WIDTH;
use crate::tui::position::{Point, Size};
use crate::tui::traits::Draw;
use crate::{impl_dirty, impl_resize};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, Stylize};
use crossterm::QueueableCommand;
use std::io;

pub struct Status {
    pos: Point,
    pub size: Size,
    text: String,
    dirty: bool,
}

impl Status {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            pos: (MIN_BUFFER_LIST_WIDTH + 1, height - 2).into(),
            size: (width - MIN_BUFFER_LIST_WIDTH - 1, 1).into(),
            text: "STATUS BAR".into(),
            dirty: true,
        }
    }
}

impl Draw for Status {
    fn draw(&mut self, out: &mut impl QueueableCommand) -> io::Result<()> {
        if self.dirty {
            self.dirty = false;
            let str = format!("{:width$}", self.text, width = self.size.width as usize)
                .with(Color::White)
                .on(Color::Blue);
            out.queue(MoveTo(self.pos.x, self.pos.y))?;
            out.queue(Print(str))?;
        }
        Ok(())
    }
}

impl_resize!(for Status);
impl_dirty!(for Status);
