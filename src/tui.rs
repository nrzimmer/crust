// The initial version of this file is "heavily based on"/"copied from" Tsoding 4at chat client.
// Project: https://github.com/tsoding/4at
// File: https://github.com/tsoding/4at/blob/789976acf2764bd4733b05f7d06f2cc889c1cc4c/src/client.rs
//
// For more information:
//
// Youtube playlist about the project: https://www.youtube.com/watch?v=qmmQAAJzM54&list=PLpM-Dvs8t0VZ1tPn-Qqdro3p_5s1HuMyF
// Twitch: https://www.twitch.tv/tsoding

use std::{cmp, io, thread};
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;
use std::time::Duration;

use crossterm::{event, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::style::Color;

use doublebuffer::DoubleBuffer;
use prompt::Prompt;

use crate::client::{Client, MessageDisplay};
use crate::tui::commands::CmdErr::*;
use crate::tui::commands::CmdOk::*;
use crate::tui::commands::Commands;

mod doublebuffer;
mod prompt;
mod commands;

macro_rules! todo_ui {
    () => {
        let _ = terminal::disable_raw_mode();
        println!("");
        todo!();
    };
}

struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

pub struct Tui {
    client: Rc<RefCell<Client>>,
    buffer: DoubleBuffer,
    width: usize,
    height: usize,
    chat: Vec<MessageDisplay>,
}

impl Tui {
    pub fn new(client: Rc<RefCell<Client>>) -> Self {
        thread::sleep(Duration::from_millis(1000));
        let (w, h) = terminal::size().unwrap();
        let width = w as usize;
        let height = h as usize;
        Self {
            client,
            buffer: DoubleBuffer::new(width, height),
            width,
            height,
            chat: Vec::new(),
        }
    }

    fn init(&self) -> io::Result<()> {
        terminal::enable_raw_mode()
    }

    fn deinit(&self) -> io::Result<()> {
        let res = terminal::disable_raw_mode();
        println!();
        res
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.init()?;

        let mut command_parser = Commands::new(self.client.clone());
        let mut stdout = stdout();
        let mut prompt = Prompt::default();
        self.buffer.flush(&mut stdout)?;
        'main: loop {
            while event::poll(Duration::ZERO).unwrap() {
                match event::read() {
                    Ok(Event::Resize(w, h)) => {
                        self.width = w as usize;
                        self.height = h as usize;
                        self.buffer.resize(self.width, self.height);
                        self.buffer.flush(&mut stdout)?;
                    }
                    Ok(Event::Paste(data)) => prompt.insert_str(&data),
                    Ok(Event::Key(event)) => {
                        if event.kind == KeyEventKind::Press {
                            match event.code {
                                KeyCode::Char(x) => {
                                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                                        match x {
                                            'c' => { break 'main; }
                                            'k' => prompt.delete_until_end(),
                                            'w' => prompt.remove_left_word(),
                                            _ => {}
                                        }
                                    } else {
                                        prompt.insert(x);
                                    }
                                }
                                KeyCode::Up => {
                                    //Todo - implement prompt history
                                }
                                KeyCode::Down => {
                                    //Todo - implement prompt history
                                }
                                KeyCode::Left => {
                                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                                        prompt.left_word();
                                    } else {
                                        prompt.left_char();
                                    }
                                }
                                KeyCode::Right => {
                                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                                        prompt.right_word();
                                    } else {
                                        prompt.right_char();
                                    }
                                }
                                KeyCode::Backspace => {
                                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                                        prompt.remove_left_word();
                                    } else {
                                        prompt.backspace()
                                    }
                                }
                                KeyCode::Delete => {
                                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                                        prompt.remove_right_word();
                                    } else {
                                        prompt.delete()
                                    }
                                }
                                KeyCode::Home => {
                                    prompt.home();
                                }
                                KeyCode::End => {
                                    prompt.end();
                                }
                                KeyCode::Enter => {
                                    let result = command_parser.try_run(&prompt.buffer);
                                    match result {
                                        Ok(success) => {
                                            match success {
                                                Ran => {}
                                                Print(_text) => { todo_ui!(); }
                                                Help(_first, _second) => { todo_ui!(); }
                                                Quit => { return self.deinit(); }
                                            }
                                        }
                                        Err(error) => {
                                            match error {
                                                NotConnected => { todo_ui!(); }
                                                AlreadyConnected => { todo_ui!(); }
                                                InvalidParameters => { todo_ui!(); }
                                                InvalidCommand(_cmd) => { todo_ui!(); }
                                                HelpNotFound => { todo_ui!(); }
                                                NotACommand => {
                                                    if self.client.borrow().is_connected() {
                                                        // This is a message
                                                        // Send it to WarPigs (for now)
                                                        let msg = prompt.buffer.iter().collect::<String>();
                                                        self.chat.push(Ok(format!("EU: {msg}")));
                                                        self.client.borrow_mut().send_message("WarPigs".to_string(), msg);
                                                    } else {
                                                        self.chat.push(Err("Not connected".to_string()));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    prompt.clear();
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }

            self.chat.append(&mut self.client.borrow_mut().process());

            self.buffer.clear();
            self.status_bar("Crust", 0, 0, self.width.into());
            // TODO: vertical scrolling for chat window
            if let Some(h) = self.height.checked_sub(3) {
                let boundary = Rect {
                    x: 0,
                    y: 1,
                    w: self.width,
                    h,
                };

                let n = self.chat.len();
                let m = n.checked_sub(boundary.h).unwrap_or(0);
                for (dy, line) in self.chat.iter().skip(m).enumerate() {
                    let (line, color) = match line {
                        Ok(x) => { (x, Color::White) }
                        Err(x) => { (x, Color::Red) }
                    };
                    let line_chars: Vec<_> = line.chars().collect();
                    self.buffer.put_cells(
                        boundary.x, boundary.y + dy,
                        line_chars.get(0..boundary.w).unwrap_or(&line_chars),
                        color, Color::Black);
                }
            }
            let status_label = if self.client.borrow().is_connected() {
                "Status: Online"
            } else {
                "Status: Offline"
            };
            if let Some(h) = self.height.checked_sub(2) {
                self.status_bar(status_label, 0, h as usize, self.width.into());
            }
            if let Some(y) = self.height.checked_sub(1) {
                let x = 1;
                if let Some(w) = self.width.checked_sub(1) {
                    prompt.render(&mut self.buffer, x, y as usize, w as usize);
                }
                self.buffer.put_cell(0, y as usize, '-', Color::White, Color::Black);
            }

            self.buffer.update(&mut stdout)?;

            if let Some(y) = self.height.checked_sub(1) {
                let x = 1;
                if let Some(w) = self.width.checked_sub(1) {
                    let _ = prompt.sync_terminal_cursor(&mut stdout, x, y as usize, w as usize);
                }
            }
            stdout.flush()?;
            self.buffer.swap();

            thread::sleep(Duration::from_millis(33));
        }

        self.deinit()
    }

    fn status_bar(&mut self, label: &str, x: usize, y: usize, w: usize) {
        let label_chars: Vec<_> = label.chars().collect();
        let n = cmp::min(label_chars.len(), w);
        self.buffer.put_cells(x, y, &label_chars[..n], Color::Black, Color::White);
        for x in label.len()..w {
            self.buffer.put_cell(x, y, ' ', Color::Black, Color::White);
        }
    }
}
