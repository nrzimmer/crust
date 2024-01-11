use crate::client::Client;
use crate::tui::commands::CmdErr;
use crate::tui::commands::CmdOk;
use crate::tui::commands::CommandParser;
use crate::tui::constants::MIN_CHAT_WIDTH;
use crate::tui::traits::{Dirty, Draw, Resize};
use crate::tui::widgets::bufferlist::BufferList;
use crate::tui::widgets::chat::Chat;
use crate::tui::widgets::nicklist::NickList;
use crate::tui::widgets::prompt::Prompt;
use crate::tui::widgets::status::Status;
use crate::tui::widgets::topic::Topic;
use crate::tui::widgets::vertbar::{VertBar, VertBarType};
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::style::{Color, Print, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{event, QueueableCommand};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::time::Duration;
use std::{io, thread};

mod commands;
mod constants;
mod position;
mod traits;
mod widgets;

macro_rules! todo_ui {
    () => {
        let _ = crossterm::terminal::disable_raw_mode();
        println!();
        todo!();
    };
    ($value:expr) => {
        let _ = crossterm::terminal::disable_raw_mode();
        println!();
        println!("{:#?}", $value);
        todo!();
    };
}
pub struct Window {
    buffer_list: BufferList,
    topic: Topic,
    chat: Chat,
    nicks: NickList,
    status: Status,
    prompt: Prompt,
    left_bar: VertBar,
    right_bar: VertBar,
    width: u16,
    height: u16,
    out: io::Stdout,
    client: Rc<RefCell<Client>>,
    parser: CommandParser,
}

macro_rules! set_all_dirty {
    ($self:ident) => {
        $self.buffer_list.dirty();
        $self.topic.dirty();
        $self.chat.dirty();
        $self.nicks.dirty();
        $self.status.dirty();
        $self.prompt.dirty();
        $self.left_bar.dirty();
        $self.right_bar.dirty();
    };
}

impl Window {
    pub fn new(width: u16, height: u16, client: Rc<RefCell<Client>>) -> Self {
        let client_clone = client.clone();
        let mut result = Self {
            buffer_list: BufferList::new(width, height),
            topic: Topic::new(width, height),
            chat: Chat::new(width, height),
            nicks: NickList::new(width, height),
            status: Status::new(width, height),
            prompt: Prompt::new(width, height),
            left_bar: VertBar::new(width, height, VertBarType::Left),
            right_bar: VertBar::new(width, height, VertBarType::Right),
            width,
            height,
            out: std::io::stdout(),
            client,
            parser: CommandParser::new(client_clone),
        };
        let _ = result.resize(width, height);
        result
    }

    fn can_draw(&self) -> bool {
        let needed_width = self.buffer_list.size.width + self.nicks.size.width + MIN_CHAT_WIDTH + 2; // 2 - vertical bars
        self.width >= needed_width && self.height >= 16
    }

    pub fn draw_terminal_too_small(&mut self) -> io::Result<()> {
        self.out.queue(Clear(ClearType::All))?;
        let text = " Terminal too small ".on(Color::Red).with(Color::White).bold();
        self.out.queue(MoveTo(self.width / 2 - 10, self.height / 2))?;
        self.out.queue(Print(text))?;
        self.out.queue(crossterm::cursor::Hide)?;
        Ok(())
    }

    pub fn draw(&mut self) -> io::Result<()> {
        if self.can_draw() {
            self.buffer_list.draw(&mut self.out)?;
            self.topic.draw(&mut self.out)?;
            self.chat.draw(&mut self.out)?;
            self.nicks.draw(&mut self.out)?;
            self.status.draw(&mut self.out)?;
            self.prompt.draw(&mut self.out)?;
        } else {
            self.draw_terminal_too_small()?;
        }
        self.out.flush()?;
        Ok(())
    }

    pub fn resize(&mut self, width: u16, height: u16) -> io::Result<()> {
        self.width = width;
        self.height = height;

        self.buffer_list.resize(self.buffer_list.pos, self.buffer_list.size.set_height(self.height));

        self.left_bar
            .resize(self.left_bar.pos.set_x(self.buffer_list.size.width), self.left_bar.size.set_height(self.height));

        self.topic.resize(
            self.topic.pos.set_x(self.left_bar.pos.x + 1),
            self.topic.size.set_width(self.width - self.topic.pos.x),
        );

        self.nicks.resize(
            self.nicks.pos.set_x(self.width - self.nicks.size.width),
            self.nicks.size.set_height(self.height - 3), // topic + status + prompt
        );

        self.right_bar.resize(
            self.right_bar.pos.set_x(self.nicks.pos.x - 1),
            self.right_bar.size.set_height(self.nicks.size.height),
        );

        self.chat.resize(
            self.chat.pos.set_x(self.topic.pos.x),
            (self.right_bar.pos.x - self.chat.pos.x, self.right_bar.size.height).into(),
        );

        self.status
            .resize((self.chat.pos.x, self.height - 2).into(), self.status.size.set_width(self.topic.size.width));

        self.prompt
            .resize((self.chat.pos.x, self.height - 1).into(), self.prompt.size.set_width(self.status.size.width));

        self.out.queue(Clear(ClearType::All))?;
        set_all_dirty!(self);
        if self.can_draw() {
            self.left_bar.draw(&mut self.out)?;
            self.right_bar.draw(&mut self.out)?;
            self.draw()?;
            self.out.flush()?;
        } else {
            self.draw_terminal_too_small()?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        while self.poll() {
            //Poll Client
            self.client.borrow_mut().poll().iter().for_each(|s| self.chat.append(s.clone()));

            self.draw()?;

            thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }

    fn poll(&mut self) -> bool {
        while event::poll(Duration::ZERO).unwrap() {
            match event::read() {
                Ok(Event::Resize(w, h)) => {
                    if let Err(_) = self.resize(w, h) {
                        return false;
                    }
                }
                Ok(Event::Key(event)) => {
                    if event.kind == KeyEventKind::Press {
                        if let KeyCode::Char(ch) = event.code {
                            if event.modifiers.contains(KeyModifiers::CONTROL) {
                                if 'c' == ch {
                                    return false;
                                }
                            }
                        }
                        if let Some(text) = self.prompt.key_press(event) {
                            if self.parse(text) == CmdOk::Quit {
                                return false;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        true
    }

    fn parse(&mut self, command: String) -> CmdOk {
        let result = self.parser.try_run(&command);
        match result {
            Ok(success) => match success {
                CmdOk::Ran => {}
                CmdOk::Print(_text) => {
                    todo_ui!();
                }
                CmdOk::Help(_first, _second) => {
                    todo_ui!();
                }
                CmdOk::Quit => {
                    return CmdOk::Quit;
                }
            },
            Err(error) => {
                match error {
                    CmdErr::NotConnected => {
                        todo_ui!();
                    }
                    CmdErr::AlreadyConnected => {
                        todo_ui!();
                    }
                    CmdErr::InvalidParameters => {
                        todo_ui!();
                    }
                    CmdErr::InvalidCommand(cmd) => {
                        todo_ui!(CmdErr::InvalidCommand(cmd));
                    }
                    CmdErr::HelpNotFound => {
                        todo_ui!();
                    }
                    CmdErr::NotACommand => {
                        if self.client.borrow().is_connected() {
                            // This is a message
                            // Send it to WarPigs (for now)
                            let msg = self.client.borrow_mut().send_message("WarPigs".to_string(), command);
                            self.chat.append(msg);
                        } else {
                            self.chat.append("Not connected".into());
                        }
                    }
                }
            }
        }
        //self.chat.append(command);
        CmdOk::Ran
    }
}
