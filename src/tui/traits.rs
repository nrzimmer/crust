use crate::tui::position::{Point, Size};
use crossterm::QueueableCommand;
use std::io;

pub trait Draw {
    fn draw(&mut self, out: &mut impl QueueableCommand) -> io::Result<()>;
}

pub trait Resize {
    fn resize(&mut self, pos: Point, size: Size);
}

pub trait Dirty {
    fn dirty(&mut self);
    fn clean(&mut self);
}

#[macro_export]
macro_rules! impl_resize {
    (for $struct_name:ident) => {
        use crate::tui::traits::Resize;
        impl Resize for $struct_name {
            fn resize(&mut self, pos: Point, size: Size) {
                self.size = size;
                self.pos = pos;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_dirty {
    (for $struct_name:ident) => {
        use crate::tui::traits::Dirty;
        impl Dirty for $struct_name {
            fn dirty(&mut self) {
                self.dirty = true;
            }
            fn clean(&mut self) {
                self.dirty = false;
            }
        }
    };
}
