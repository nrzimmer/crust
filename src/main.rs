use crate::client::{Client, UserInfo};
use crate::config::Config;
use crossterm::cursor::MoveTo;
use crossterm::{terminal, QueueableCommand};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

mod app;
mod client;
mod config;
mod tui;

pub fn main() -> Result<(), usize> {
    let config = match Config::load() {
        Some(config) => {
            println!("{config:?}");
            config
        }
        None => match config::request_config() {
            Some(config) => {
                config.save();
                config
            }
            None => {
                eprintln!("Invalid config provided");
                return Err(1);
            }
        },
    };

    let ref user = config.user.unwrap();
    let client = Client::new(UserInfo::new(user.nicknames.iter().next().unwrap().clone(), user.username.clone(), user.realname.clone()).unwrap());

    thread::sleep(Duration::from_millis(1000));
    let (w, h) = terminal::size().unwrap();
    for _ in 0..h {
        println!();
    }

    let client = Rc::new(RefCell::new(client));

    let _ = terminal::enable_raw_mode();

    let mut frame = tui::Window::new(w, h, client);
    if let Err(e) = frame.run() {
        finalize(h);
        println!("{e}");
        return Err(3);
    }

    finalize(h);
    Ok(())
}

fn finalize(h: u16) {
    let mut out = std::io::stdout();
    let _ = out.queue(MoveTo(0, h + 1));
    let _ = out.flush();
    let _ = terminal::disable_raw_mode();
}
