use std::cell::RefCell;
use std::rc::Rc;

use config::Config;

use crate::client::{Client, UserInfo};
use crate::tui::Tui;

mod client;
mod tui;
mod app;
mod config;

fn main() -> Result<(), usize> {
    let config = match Config::load() {
        Some(config) => {
            println!("{config:?}");
            config
        }
        None => {
            match config::request_config() {
                Some(config) => {
                    config.save();
                    config
                }
                None => {
                    eprintln!("Invalid config provided");
                    return Err(1);
                }
            }
        }
    };

    let ref user = config.user.unwrap();
    let client = Client::new(
        UserInfo::new(
            user.nicknames.iter().next().unwrap().clone(),
            user.username.clone(),
            user.realname.clone(),
        ).unwrap(),
    );

    let mut ui: Tui = Tui::new(Rc::new(RefCell::new(client)));

    match ui.run() {
        Ok(_) => { Ok(()) }
        Err(_) => { Err(2) }
    }
}
