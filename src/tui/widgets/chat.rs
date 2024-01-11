use crate::tui::constants::{MIN_BUFFER_LIST_WIDTH, MIN_NICK_LIST_WIDTH};
use crate::tui::position::{Point, Size};
use crate::tui::traits::Draw;
use crate::{impl_dirty, impl_resize};
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::QueueableCommand;
use message::Message;
use std::io;

pub mod message;

pub struct Chat {
    pub pos: Point,
    size: Size,
    content: Vec<Message>,
    dirty: bool,
}

impl Chat {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            pos: (MIN_BUFFER_LIST_WIDTH + 1, 1).into(),
            size: (width - MIN_BUFFER_LIST_WIDTH - MIN_NICK_LIST_WIDTH - 2, height - 3).into(),
            content: Vec::new(),
            dirty: true,
        }
    }

    pub fn append(&mut self, text: String) {
        self.dirty = true;
        self.content.push(text.into());
    }
}

impl Draw for Chat {
    fn draw(&mut self, out: &mut impl QueueableCommand) -> io::Result<()> {
        if self.dirty {
            self.dirty = false;
            let mut screen: Vec<String> = Vec::new();
            for item in self.content.clone().iter_mut() {
                match item {
                    Message::Info { message } => {
                        let mut lines: Vec<String> = split_lines_with_max_len(&message, self.size.width as usize);
                        screen.append(&mut lines);
                    }
                    Message::FromUser { .. } => {}
                    Message::Join { .. } => {}
                    Message::Leave { .. } => {}
                    Message::Quick { .. } => {}
                    Message::ChangeDay { .. } => {}
                    Message::Mode { .. } => {}
                }
            }
            if screen.len() > self.size.height as usize {
                for i in (0..self.size.height).rev() {
                    out.queue(MoveTo(self.pos.x, self.pos.y + i))?;
                    out.queue(Print(screen.pop().unwrap()))?;
                }
            } else {
                for i in 0..screen.len() {
                    out.queue(MoveTo(self.pos.x, self.pos.y + i as u16))?;
                    out.queue(Print(screen.get(i).unwrap()))?;
                }
            }
        }
        Ok(())
    }
}

fn split_lines_with_max_len(input: &str, max_len: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::with_capacity(max_len);

    for word in input.split_whitespace() {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= max_len {
            current_line.push_str(&format!(" {}", word));
        } else {
            result.push(format!("{:max_len$}", current_line));
            current_line.clear();
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        result.push(format!("{:max_len$}", current_line));
    }

    result
}

impl_resize!(for Chat);
impl_dirty!(for Chat);
