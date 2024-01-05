// The initial version of this file is "heavily based on"/"copied from" Tsoding 4at chat client.
// Project: https://github.com/tsoding/4at
// File: https://github.com/tsoding/4at/blob/789976acf2764bd4733b05f7d06f2cc889c1cc4c/src/client.rs
//
// For more information:
//
// Youtube playlist about the project: https://www.youtube.com/watch?v=qmmQAAJzM54&list=PLpM-Dvs8t0VZ1tPn-Qqdro3p_5s1HuMyF
// Twitch: https://www.twitch.tv/tsoding

use std::cell::RefCell;
use std::rc::Rc;

use crate::client::Client;
use crate::tui::commands::CmdErr::*;
use crate::tui::commands::CmdOk::*;

pub enum CmdOk {
    Ran,
    Print(&'static str),
    Help(&'static str, &'static str),
    Quit,
}

pub enum CmdErr {
    NotACommand,
    NotConnected,
    AlreadyConnected,
    InvalidParameters,
    InvalidCommand(&'static str),
    HelpNotFound,
}

pub type CommandResult = Result<CmdOk, CmdErr>;
type CommandFunc = fn(&mut Commands, &str) -> CommandResult;

struct Command {
    name: &'static str,
    description: &'static str,
    signature: &'static str,
    run: CommandFunc,
}

pub struct Commands {
    client: Rc<RefCell<Client>>,
    cmd_list: Vec<Command>,
}

impl Commands {
    pub fn new(client: Rc<RefCell<Client>>) -> Self {
        let mut result = Commands {
            cmd_list: Vec::new(),
            client,
        };

        result.register("join", "Join a channel", "/join <channel>", Self::join);
        result.register("j", "Join a channel", "/j <channel>", Self::join);

        result.register("c", "connects to quakenet", "/c", Self::connect_quakenet);
        result.register(
            "connect",
            "Connect to a server at <ip> and <port>",
            "/connect <ip> <port>",
            Self::connect,
        );
        result.register("quit", "Close the chat", "/quit", Self::quit);
        result.register("help", "Print help", "/help [command]", Self::help);

        result
    }

    fn connect(&mut self, argument: &str) -> CommandResult {
        if self.client.borrow().is_connected() {
            return Err(AlreadyConnected);
        }

        let chunks: Vec<&str> = argument.split(' ').filter(|s| !s.is_empty()).collect();
        match &chunks[..] {
            &[ip, port] => {
                let _ = self.client.borrow_mut().connect(format!("{ip}:{port}"));
                Ok(Ran)
            }
            _ => {
                Err(InvalidParameters)
            }
        }
    }

    fn connect_quakenet(&mut self, _argument: &str) -> CommandResult {
        if self.client.borrow().is_connected() {
            return Err(AlreadyConnected);
        }

        let _ = self.client.borrow_mut().connect(format!("irc.quakenet.org:6667"));
        Ok(Ran)
    }

    fn join(&mut self, argument: &str) -> CommandResult {
        if !self.client.borrow().is_connected() {
            return Err(NotConnected);
        }

        let chunks: Vec<&str> = argument.split(' ').filter(|s| !s.is_empty()).collect();
        match &chunks[..] {
            &[channel] => {
                self.client.borrow_mut().join(channel);
            }
            _ => {
                return Err(InvalidParameters);
            }
        }
        Ok(Ran)
    }

    fn quit(&mut self, _: &str) -> CommandResult {
        Ok(Quit)
    }

    fn help(&mut self, argument: &str) -> CommandResult {
        let chunks: Vec<&str> = argument.split(' ').filter(|s| !s.is_empty()).collect();
        match &chunks[..] {
            &[command] => {
                if let Some(cmd) = self.find_command(command) {
                    return Ok(Help(cmd.description, cmd.signature));
                }
            }
            &[] => {
                todo!();
            }
            _ => {
                return Err(HelpNotFound);
            }
        }
        Ok(Ran)
    }

    fn register(
        &mut self,
        name: &'static str,
        description: &'static str,
        signature: &'static str,
        run_function: CommandFunc,
    ) {
        if let Some(cmd) = self.cmd_list.iter_mut().find(|command| command.name == name) {
            cmd.description = description;
            cmd.signature = signature;
            cmd.run = run_function;
        } else {
            self.cmd_list.push(Command {
                name,
                description,
                signature,
                run: run_function,
            });
        }
    }

    fn find_command(&self, name: &str) -> Option<&Command> {
        self.cmd_list.iter().find(|command| command.name == name)
    }

    pub fn try_run(&mut self, prompt: &[char]) -> CommandResult {//Option<(&'a [char], &'a [char])> {
        if let Some(prompt) = prompt.strip_prefix(&['/']) {
            let mut iter = prompt.splitn(2, |x| *x == ' ');
            let name = iter.next().unwrap_or(prompt).iter().collect::<String>();
            let argument = iter.next().unwrap_or(&[]).iter().collect::<String>();
            if let Some(command) = self.find_command(&name) {
                return (command.run)(self, &argument);
            } else {
                return Err(InvalidCommand("/{name}`"));
            }
        }
        Err(NotACommand)
    }
}
