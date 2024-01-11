use std::io::Write;
use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::app;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub nicknames: Vec<String>,
    pub username: String,
    pub realname: String,
}

impl User {
    pub fn new(nicknames: Vec<String>, username: String, realname: String) -> Self {
        Self { nicknames, username, realname }
    }

    pub fn clean(&self) -> Self {
        Self {
            nicknames: self
                .nicknames
                .iter()
                .map(|nick| nick.trim().to_string())
                .filter(|nick| !nick.is_empty())
                .collect::<Vec<_>>(),
            username: self.username.trim().to_string(),
            realname: self.realname.trim().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Server {
    last_nickname: Option<String>,
    name: String,
    address: String,
    port: u16,
}

impl Server {
    pub fn new(name: String, address: String, port: u16) -> Self {
        Self {
            last_nickname: None,
            name,
            address,
            port,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub user: Option<User>,
    pub servers: Option<Vec<Server>>,
}

impl Config {
    pub fn new(user: User, servers: Option<Vec<Server>>) -> Option<Self> {
        if user.nicknames.len() > 0 {
            Some(Self { user: Some(user), servers })
        } else {
            None
        }
    }

    pub fn save(&self) {
        let file_path = Self::config_file_path();
        match serde_json::to_string_pretty(self) {
            Ok(json) => match fs::write(file_path, json.into_bytes()) {
                Ok(_) => {
                    return;
                }
                Err(e) => {
                    eprintln!("Failed to save the configuration. Error: {e}.");
                }
            },
            Err(e) => {
                eprintln!("Failed to serialize the configuration. Error: {e}.");
            }
        }
    }

    pub fn load() -> Option<Self> {
        let file_path = Self::config_file_path();

        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(mut config) = serde_json::from_str::<Config>(&content) {
                if let Some(ref mut user) = config.user {
                    let clean = user.clean();

                    let cleaned = clean.ne(user);

                    if cleaned {
                        *user = clean;
                    }

                    if !user.realname.is_empty() && !user.username.is_empty() && user.nicknames.len() > 0 {
                        if cleaned {
                            config.save();
                        }
                        return Some(config);
                    }
                }
            }
        }

        None
    }

    fn config_file_path() -> String {
        format!(
            "{home}/.config/{app_name}",
            home = match std::env::var("HOME") {
                Ok(path) => path,
                Err(_) => {
                    panic!("Could not find current user home folder! $HOME variable not set!")
                }
            },
            app_name = app::name()
        )
    }
}

fn request_input(what: &'static str) -> Option<String> {
    let mut value = String::new();
    loop {
        print!("Provide yor {what}: ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Failed to flush to stdout. Error: {e}.");
            return None;
        }
        match io::stdin().read_line(&mut value) {
            Ok(size) => {
                if size > 1 {
                    value = value.trim().to_string();
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from input. Error: {e}.");
                return None;
            }
        }
        println!("Invalid {what}.");
    }

    Some(value)
}

pub fn request_config() -> Option<Config> {
    println!("To start using {} we need to know your username, real name and nickname.", app::Name());
    println!("Use CTRL+C to cancel.");
    let username: String;
    let realname: String;
    let mut nickname: String;
    let mut nicks: Vec<String> = Vec::<String>::new();

    match request_input("username") {
        None => {
            return None;
        }
        Some(value) => username = value,
    }

    match request_input("real name") {
        None => {
            return None;
        }
        Some(value) => realname = value,
    }

    loop {
        match request_input("nickname") {
            None => {
                return None;
            }
            Some(value) => nickname = value,
        }
        nicks.push(nickname);

        let mut yn = String::new();
        loop {
            print!("Do you want to provide any alternative nickname [y/n]? ");
            if let Err(e) = io::stdout().flush() {
                eprintln!("Failed to flush to stdout. Error: {e}.");
                return None;
            }
            match io::stdin().read_line(&mut yn) {
                Ok(size) => {
                    if size > 1 && size < 3 {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from input. Error: {e}.");
                    return None;
                }
            }
            println!("Invalid option.");
        }

        if yn.chars().next().unwrap().eq_ignore_ascii_case(&'n') {
            break;
        }
    }

    let user = User::new(nicks, username, realname);

    Config::new(user, None)
}
