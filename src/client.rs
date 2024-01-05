use std::fmt;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::ops::DerefMut;

use crate::client::repliestypes::Replies;
use crate::client::ringbuffer::RingBuffer;

mod ringbuffer;
mod repliestypes;

#[derive(Debug)]
pub enum ClientError {
    NoNickDefined,
    NoUserDefined,
    NoNameDefined,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            ClientError::NoNickDefined => "No nick was defined",
            ClientError::NoUserDefined => "No user was defined",
            ClientError::NoNameDefined => "No name was defined",
        };
        write!(f, "{text}")
    }
}

#[derive(Debug)]
pub struct UserInfo {
    nick: String,
    user: String,
    name: String,
}

impl UserInfo {
    pub fn new(nick: String, user: String, name: String) -> Result<Self, ClientError> {
        if nick.len() == 0 {
            return Err(ClientError::NoNickDefined);
        }
        if user.len() == 0 {
            return Err(ClientError::NoUserDefined);
        }
        if name.len() == 0 {
            return Err(ClientError::NoNameDefined);
        }
        Ok(UserInfo { nick, user, name })
    }

    fn get_user_msg(&self) -> String {
        format!("USER {} 0 * :{}", self.user, self.name).to_string()
    }

    fn get_nick_msg(&self) -> String {
        format!("NICK {}", self.nick).to_string()
    }
}

#[derive(Debug)]
pub struct MessageFromServer {
    source: String,
    reply_type: Replies,
    dest: String,
    content: String,
}

#[derive(Debug)]
pub struct Channel {
    name: String,
    topic: String,
    user_list: Vec<UserInfo>,
}

pub type MessageDisplay = Result<String, String>;

#[derive(Debug)]
pub struct Client {
    stream: Option<TcpStream>,
    buffer: RingBuffer<u8>,
    connected: bool,
    user_info: UserInfo,
    return_lines: Vec<MessageDisplay>,
}

macro_rules! chat_msg {
    ($chat:expr, $($arg:tt)*) => {
        $chat.push(Ok(format!($($arg)*)))
    }
}

macro_rules! chat_error {
    ($chat:expr, $($arg:tt)*) => {
        $chat.push(Err(format!($($arg)*)))
    }
}


impl Client {
    pub fn new(user_info: UserInfo) -> Self {
        Client {
            stream: None,
            buffer: RingBuffer::new(1024 * 10), //8kb
            connected: false,
            user_info,
            return_lines: Vec::new(),
        }
    }

    fn identify(&mut self) {
        self.send_string(self.user_info.get_nick_msg());
        self.send_string(self.user_info.get_user_msg());
    }

    pub fn join(&mut self, channel: &str) {
        self.send_string(format!("JOIN #{channel}"));
    }

    pub fn connect<A: ToSocketAddrs>(&mut self, addr: A) -> Result<(), String> {
        if self.stream.is_some() {
            return Err("Already connected".to_string());
        } else {
            match TcpStream::connect(addr) {
                Ok(stream) => match stream.set_nonblocking(true) {
                    Ok(_) => {
                        self.connected = true;
                        self.stream = Some(stream);
                        self.identify();
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                },
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        match &self.stream {
            None => false,
            Some(stream) => {
                let mut buff = [0u8];
                match stream.peek(&mut buff) {
                    Ok(n) => n > 0,
                    Err(e) => e.kind() == ErrorKind::WouldBlock,
                }
            }
        }
    }

    pub fn process(&mut self) -> Vec<MessageDisplay> {
        let mut new_data = false;
        if let Some(ref mut stream) = self.stream {
            match stream.read_vectored(self.buffer.slices().deref_mut()) {
                Ok(n) => if n > 0 {
                    self.buffer.wrote(n);
                    new_data = true;
                }
                Err(e) => if e.kind() != ErrorKind::WouldBlock {
                    chat_error!(self.return_lines, "{}", e);
                    self.connected = false;
                }
            }
        }


        if new_data {
            self.try_read_server_data();
        }

        std::mem::take(&mut self.return_lines)
    }
    fn try_read_server_data(&mut self) {
        while let Some(up_to) = self.buffer.find_first(&[13u8, 10u8]) {
            if let Some(message_bytes) = self.buffer.consume(up_to) {
                self.buffer.discard(2); // Discard the line break
                self.try_parse_server_data(String::from_utf8_lossy(&message_bytes).to_string());
            }
        }
    }

    fn try_parse_server_data(&mut self, message: String) {
        if self.process_ping(&message) { return; }
        if let Some((source, rest)) = message.strip_prefix(':').and_then(|m| m.split_once(' ')) {
            if let Some((msg_type, rest)) = rest.split_once(' ') {
                if msg_type.len() == 3 {
                    if let Some((dest, content)) = rest.split_once(' ') {
                        if self.try_parse_server_reply(source, Replies::from_str(msg_type), dest, content) {
                            return;
                        }
                    }
                } else {
                    if Self::try_parse_server_message(source, msg_type, rest) {
                        return;
                    }
                }
            }
        }

        chat_error!(self.return_lines,"<<< {message}");
    }

    fn process_ping(&mut self, message: &str) -> bool {
        if let Some(part) = message.strip_prefix("PING ") {
            if let Some(host) = part.split_whitespace().next() {
                chat_msg!(self.return_lines, "<<< {}", message);
                self.send_string(format!("PONG {}", host));
                return true;
            }
        }
        false
    }

    fn try_parse_server_message(_source: &str, _msg_type: &str, _rest: &str) -> bool {
        false
    }

    fn try_parse_server_reply(&mut self, _source: &str, msg_type: Replies, _dest: &str, content: &str) -> bool {
        match msg_type {
            Replies::RPL_WELCOME | Replies::RPL_YOURHOST | Replies::RPL_CREATED | Replies::RPL_MYINFO | Replies::RPL_BOUNCE | Replies::RPL_LUSERCLIENT | Replies::RPL_LUSERME | Replies::RPL_MOTDSTART | Replies::RPL_MOTD | Replies::RPL_ENDOFMOTD => {
                chat_msg!(self.return_lines,"{}", content.trim_start_matches([':', ' ']));
                true
            }
            _ => false,
        }
    }

    fn send_string(&mut self, command: String) {
        chat_msg!(self.return_lines, ">>> {command}");
        self.send_bytes(command.as_bytes());
    }

    #[inline]
    fn send_bytes(&mut self, command: &[u8]) {
        match self.stream {
            None => {}
            Some(ref mut stream) => {
                let _ = stream.write_all(command);
                let _ = stream.write_all(&[13u8, 10u8]);
            }
        }
    }

    pub fn send_message(&mut self, dest: String, msg: String) {
        self.send_bytes(format!("PRIVMSG #{dest} :{msg}").as_bytes());
    }
}
