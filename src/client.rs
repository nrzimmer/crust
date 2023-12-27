use crate::fifobuffer::FifoBuffer;
use std::fmt;
use std::io::{ErrorKind, Read, Write};
use std::net::Shutdown::Both;
use std::net::{TcpStream, ToSocketAddrs};
use std::ops::DerefMut;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum Replies {
    RPL_WELCOME,
    RPL_YOURHOST,
    RPL_CREATED,
    RPL_MYINFO,
    RPL_BOUNCE,
    RPL_USERHOST,
    RPL_ISON,
    RPL_AWAY,
    RPL_UNAWAY,
    RPL_NOWAWAY,
    RPL_WHOISUSER,
    RPL_WHOISSERVER,
    RPL_WHOISOPERATOR,
    RPL_WHOISIDLE,
    RPL_ENDOFWHOIS,
    RPL_WHOISCHANNELS,
    RPL_WHOWASUSER,
    RPL_ENDOFWHOWAS,
    RPL_LISTSTART,
    RPL_LIST,
    RPL_LISTEND,
    RPL_UNIQOPIS,
    RPL_CHANNELMODEIS,
    RPL_NOTOPIC,
    RPL_TOPIC,
    RPL_INVITING,
    RPL_SUMMONING,
    RPL_INVITELIST,
    RPL_ENDOFINVITELIST,
    RPL_EXCEPTLIST,
    RPL_ENDOFEXCEPTLIST,
    RPL_VERSION,
    RPL_WHOREPLY,
    RPL_ENDOFWHO,
    RPL_NAMREPLY,
    RPL_ENDOFNAMES,
    RPL_LINKS,
    RPL_ENDOFLINKS,
    RPL_BANLIST,
    RPL_ENDOFBANLIST,
    RPL_INFO,
    RPL_ENDOFINFO,
    RPL_MOTDSTART,
    RPL_MOTD,
    RPL_ENDOFMOTD,
    RPL_YOUREOPER,
    RPL_REHASHING,
    RPL_YOURESERVICE,
    RPL_TIME,
    RPL_USERSSTART,
    RPL_USERS,
    RPL_ENDOFUSERS,
    RPL_NOUSERS,
    RPL_TRACELINK,
    RPL_TRACECONNECTING,
    RPL_TRACEHANDSHAKE,
    RPL_TRACEUNKNOWN,
    RPL_TRACEOPERATOR,
    RPL_TRACEUSER,
    RPL_TRACESERVER,
    RPL_TRACESERVICE,
    RPL_TRACENEWTYPE,
    RPL_TRACECLASS,
    RPL_TRACERECONNECT,
    RPL_TRACELOG,
    RPL_TRACEEND,
    RPL_STATSLINKINFO,
    RPL_STATSCOMMANDS,
    RPL_ENDOFSTATS,
    RPL_STATSUPTIME,
    RPL_STATSOLINE,
    RPL_UMODEIS,
    RPL_SERVLIST,
    RPL_SERVLISTEND,
    RPL_LUSERCLIENT,
    RPL_LUSEROP,
    RPL_LUSERUNKNOWN,
    RPL_LUSERCHANNELS,
    RPL_LUSERME,
    RPL_ADMINME,
    RPL_ADMINLOC1,
    RPL_ADMINLOC2,
    RPL_ADMINEMAIL,
    RPL_TRYAGAIN,
    ERR_NOSUCHNICK,
    ERR_NOSUCHSERVER,
    ERR_NOSUCHCHANNEL,
    ERR_CANNOTSENDTOCHAN,
    ERR_TOOMANYCHANNELS,
    ERR_WASNOSUCHNICK,
    ERR_TOOMANYTARGETS,
    ERR_NOSUCHSERVICE,
    ERR_NOORIGIN,
    ERR_NORECIPIENT,
    ERR_NOTEXTTOSEND,
    ERR_NOTOPLEVEL,
    ERR_WILDTOPLEVEL,
    ERR_BADMASK,
    ERR_UNKNOWNCOMMAND,
    ERR_NOMOTD,
    ERR_NOADMININFO,
    ERR_FILEERROR,
    ERR_NONICKNAMEGIVEN,
    ERR_ERRONEUSNICKNAME,
    ERR_NICKNAMEINUSE,
    ERR_NICKCOLLISION,
    ERR_UNAVAILRESOURCE,
    ERR_USERNOTINCHANNEL,
    ERR_NOTONCHANNEL,
    ERR_USERONCHANNEL,
    ERR_NOLOGIN,
    ERR_SUMMONDISABLED,
    ERR_USERSDISABLED,
    ERR_NOTREGISTERED,
    ERR_NEEDMOREPARAMS,
    ERR_ALREADYREGISTRED,
    ERR_NOPERMFORHOST,
    ERR_PASSWDMISMATCH,
    ERR_YOUREBANNEDCREEP,
    ERR_YOUWILLBEBANNED,
    ERR_KEYSET,
    ERR_CHANNELISFULL,
    ERR_UNKNOWNMODE,
    ERR_INVITEONLYCHAN,
    ERR_BANNEDFROMCHAN,
    ERR_BADCHANNELKEY,
    ERR_BADCHANMASK,
    ERR_NOCHANMODES,
    ERR_BANLISTFULL,
    ERR_NOPRIVILEGES,
    ERR_CHANOPRIVSNEEDED,
    ERR_CANTKILLSERVER,
    ERR_RESTRICTED,
    ERR_UNIQOPPRIVSNEEDED,
    ERR_NOOPERHOST,
    ERR_UMODEUNKNOWNFLAG,
    ERR_USERSDONTMATCH,
    RPL_SERVICEINFO,
    RPL_ENDOFSERVICES,
    RPL_SERVICE,
    RPL_NONE,
    RPL_WHOISCHANOP,
    RPL_KILLDONE,
    RPL_CLOSING,
    RPL_CLOSEEND,
    RPL_INFOSTART,
    RPL_MYPORTIS,
    RPL_STATSCLINE,
    RPL_STATSNLINE,
    RPL_STATSILINE,
    RPL_STATSKLINE,
    RPL_STATSQLINE,
    RPL_STATSYLINE,
    RPL_STATSVLINE,
    RPL_STATSLLINE,
    RPL_STATSHLINE,
    RPL_STATSSLINE,
    RPL_STATSPING,
    RPL_STATSBLINE,
    RPL_STATSDLINE,
    ERR_NOSERVICEHOST,
    UNKNOWN(String),
}

#[allow(dead_code)]
impl Replies {
    pub fn from_str(numbers: &str) -> Replies {
        match numbers {
            "001" => Replies::RPL_WELCOME,
            "002" => Replies::RPL_YOURHOST,
            "003" => Replies::RPL_CREATED,
            "004" => Replies::RPL_MYINFO,
            "005" => Replies::RPL_BOUNCE,
            "302" => Replies::RPL_USERHOST,
            "303" => Replies::RPL_ISON,
            "301" => Replies::RPL_AWAY,
            "305" => Replies::RPL_UNAWAY,
            "306" => Replies::RPL_NOWAWAY,
            "311" => Replies::RPL_WHOISUSER,
            "312" => Replies::RPL_WHOISSERVER,
            "313" => Replies::RPL_WHOISOPERATOR,
            "317" => Replies::RPL_WHOISIDLE,
            "318" => Replies::RPL_ENDOFWHOIS,
            "319" => Replies::RPL_WHOISCHANNELS,
            "314" => Replies::RPL_WHOWASUSER,
            "369" => Replies::RPL_ENDOFWHOWAS,
            "321" => Replies::RPL_LISTSTART,
            "322" => Replies::RPL_LIST,
            "323" => Replies::RPL_LISTEND,
            "325" => Replies::RPL_UNIQOPIS,
            "324" => Replies::RPL_CHANNELMODEIS,
            "331" => Replies::RPL_NOTOPIC,
            "332" => Replies::RPL_TOPIC,
            "341" => Replies::RPL_INVITING,
            "342" => Replies::RPL_SUMMONING,
            "346" => Replies::RPL_INVITELIST,
            "347" => Replies::RPL_ENDOFINVITELIST,
            "348" => Replies::RPL_EXCEPTLIST,
            "349" => Replies::RPL_ENDOFEXCEPTLIST,
            "351" => Replies::RPL_VERSION,
            "352" => Replies::RPL_WHOREPLY,
            "315" => Replies::RPL_ENDOFWHO,
            "353" => Replies::RPL_NAMREPLY,
            "366" => Replies::RPL_ENDOFNAMES,
            "364" => Replies::RPL_LINKS,
            "365" => Replies::RPL_ENDOFLINKS,
            "367" => Replies::RPL_BANLIST,
            "368" => Replies::RPL_ENDOFBANLIST,
            "371" => Replies::RPL_INFO,
            "374" => Replies::RPL_ENDOFINFO,
            "375" => Replies::RPL_MOTDSTART,
            "372" => Replies::RPL_MOTD,
            "376" => Replies::RPL_ENDOFMOTD,
            "381" => Replies::RPL_YOUREOPER,
            "382" => Replies::RPL_REHASHING,
            "383" => Replies::RPL_YOURESERVICE,
            "391" => Replies::RPL_TIME,
            "392" => Replies::RPL_USERSSTART,
            "393" => Replies::RPL_USERS,
            "394" => Replies::RPL_ENDOFUSERS,
            "395" => Replies::RPL_NOUSERS,
            "200" => Replies::RPL_TRACELINK,
            "201" => Replies::RPL_TRACECONNECTING,
            "202" => Replies::RPL_TRACEHANDSHAKE,
            "203" => Replies::RPL_TRACEUNKNOWN,
            "204" => Replies::RPL_TRACEOPERATOR,
            "205" => Replies::RPL_TRACEUSER,
            "206" => Replies::RPL_TRACESERVER,
            "207" => Replies::RPL_TRACESERVICE,
            "208" => Replies::RPL_TRACENEWTYPE,
            "209" => Replies::RPL_TRACECLASS,
            "210" => Replies::RPL_TRACERECONNECT,
            "261" => Replies::RPL_TRACELOG,
            "262" => Replies::RPL_TRACEEND,
            "211" => Replies::RPL_STATSLINKINFO,
            "212" => Replies::RPL_STATSCOMMANDS,
            "219" => Replies::RPL_ENDOFSTATS,
            "242" => Replies::RPL_STATSUPTIME,
            "243" => Replies::RPL_STATSOLINE,
            "221" => Replies::RPL_UMODEIS,
            "234" => Replies::RPL_SERVLIST,
            "235" => Replies::RPL_SERVLISTEND,
            "251" => Replies::RPL_LUSERCLIENT,
            "252" => Replies::RPL_LUSEROP,
            "253" => Replies::RPL_LUSERUNKNOWN,
            "254" => Replies::RPL_LUSERCHANNELS,
            "255" => Replies::RPL_LUSERME,
            "256" => Replies::RPL_ADMINME,
            "257" => Replies::RPL_ADMINLOC1,
            "258" => Replies::RPL_ADMINLOC2,
            "259" => Replies::RPL_ADMINEMAIL,
            "263" => Replies::RPL_TRYAGAIN,
            "401" => Replies::ERR_NOSUCHNICK,
            "402" => Replies::ERR_NOSUCHSERVER,
            "403" => Replies::ERR_NOSUCHCHANNEL,
            "404" => Replies::ERR_CANNOTSENDTOCHAN,
            "405" => Replies::ERR_TOOMANYCHANNELS,
            "406" => Replies::ERR_WASNOSUCHNICK,
            "407" => Replies::ERR_TOOMANYTARGETS,
            "408" => Replies::ERR_NOSUCHSERVICE,
            "409" => Replies::ERR_NOORIGIN,
            "411" => Replies::ERR_NORECIPIENT,
            "412" => Replies::ERR_NOTEXTTOSEND,
            "413" => Replies::ERR_NOTOPLEVEL,
            "414" => Replies::ERR_WILDTOPLEVEL,
            "415" => Replies::ERR_BADMASK,
            "421" => Replies::ERR_UNKNOWNCOMMAND,
            "422" => Replies::ERR_NOMOTD,
            "423" => Replies::ERR_NOADMININFO,
            "424" => Replies::ERR_FILEERROR,
            "431" => Replies::ERR_NONICKNAMEGIVEN,
            "432" => Replies::ERR_ERRONEUSNICKNAME,
            "433" => Replies::ERR_NICKNAMEINUSE,
            "436" => Replies::ERR_NICKCOLLISION,
            "437" => Replies::ERR_UNAVAILRESOURCE,
            "441" => Replies::ERR_USERNOTINCHANNEL,
            "442" => Replies::ERR_NOTONCHANNEL,
            "443" => Replies::ERR_USERONCHANNEL,
            "444" => Replies::ERR_NOLOGIN,
            "445" => Replies::ERR_SUMMONDISABLED,
            "446" => Replies::ERR_USERSDISABLED,
            "451" => Replies::ERR_NOTREGISTERED,
            "461" => Replies::ERR_NEEDMOREPARAMS,
            "462" => Replies::ERR_ALREADYREGISTRED,
            "463" => Replies::ERR_NOPERMFORHOST,
            "464" => Replies::ERR_PASSWDMISMATCH,
            "465" => Replies::ERR_YOUREBANNEDCREEP,
            "466" => Replies::ERR_YOUWILLBEBANNED,
            "467" => Replies::ERR_KEYSET,
            "471" => Replies::ERR_CHANNELISFULL,
            "472" => Replies::ERR_UNKNOWNMODE,
            "473" => Replies::ERR_INVITEONLYCHAN,
            "474" => Replies::ERR_BANNEDFROMCHAN,
            "475" => Replies::ERR_BADCHANNELKEY,
            "476" => Replies::ERR_BADCHANMASK,
            "477" => Replies::ERR_NOCHANMODES,
            "478" => Replies::ERR_BANLISTFULL,
            "481" => Replies::ERR_NOPRIVILEGES,
            "482" => Replies::ERR_CHANOPRIVSNEEDED,
            "483" => Replies::ERR_CANTKILLSERVER,
            "484" => Replies::ERR_RESTRICTED,
            "485" => Replies::ERR_UNIQOPPRIVSNEEDED,
            "491" => Replies::ERR_NOOPERHOST,
            "501" => Replies::ERR_UMODEUNKNOWNFLAG,
            "502" => Replies::ERR_USERSDONTMATCH,
            "231" => Replies::RPL_SERVICEINFO,
            "232" => Replies::RPL_ENDOFSERVICES,
            "233" => Replies::RPL_SERVICE,
            "300" => Replies::RPL_NONE,
            "316" => Replies::RPL_WHOISCHANOP,
            "361" => Replies::RPL_KILLDONE,
            "362" => Replies::RPL_CLOSING,
            "363" => Replies::RPL_CLOSEEND,
            "373" => Replies::RPL_INFOSTART,
            "384" => Replies::RPL_MYPORTIS,
            "213" => Replies::RPL_STATSCLINE,
            "214" => Replies::RPL_STATSNLINE,
            "215" => Replies::RPL_STATSILINE,
            "216" => Replies::RPL_STATSKLINE,
            "217" => Replies::RPL_STATSQLINE,
            "218" => Replies::RPL_STATSYLINE,
            "240" => Replies::RPL_STATSVLINE,
            "241" => Replies::RPL_STATSLLINE,
            "244" => Replies::RPL_STATSHLINE,
            //"244" => Replies::RPL_STATSSLINE,
            "246" => Replies::RPL_STATSPING,
            "247" => Replies::RPL_STATSBLINE,
            "250" => Replies::RPL_STATSDLINE,
            "492" => Replies::ERR_NOSERVICEHOST,
            _ => Replies::UNKNOWN(numbers.to_string()),
        }
    }
}

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

pub struct UserInfo {
    nick: Vec<String>,
    user: String,
    name: String,
}

impl UserInfo {
    pub fn new(nick: Vec<String>, user: String, name: String) -> Result<Self, ClientError> {
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
        format!("USER {} 0 * :{}\r\n", self.user, self.name).to_string()
    }

    fn get_nick_msg(&self) -> String {
        format!("NICK {}\r\n", self.nick.get(0).unwrap()).to_string()
    }
}

pub struct Client {
    stream: Option<TcpStream>,
    buffer: FifoBuffer<u8>,
    connected: bool,
    user_info: UserInfo,
}

impl Client {
    pub fn new(user_info: UserInfo) -> Self {
        Client {
            stream: None,
            buffer: FifoBuffer::new(1024 * 10), //8kb
            connected: false,
            user_info,
        }
    }

    fn identify(&mut self) {
        self.send_string(self.user_info.get_nick_msg());
        self.send_string(self.user_info.get_user_msg());
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

    #[allow(dead_code)]
    pub fn disconnect(&mut self) {
        match &self.stream {
            None => {}
            Some(stream) => {
                let _ = stream.shutdown(Both);
                self.stream = None;
                self.connected = false;
            }
        }
    }

    pub fn process(&mut self) {
        let mut new_data = false;
        match self.stream {
            None => {}
            Some(ref mut stream) => {
                match stream.read_vectored(self.buffer.get_vector_for_writing().deref_mut()) {
                    Ok(n) => {
                        if n > 0 {
                            self.buffer.wrote(n);
                            new_data = true;
                        }
                    }
                    Err(e) => {
                        if e.kind() != ErrorKind::WouldBlock {
                            eprintln!("{}", e.to_string());
                            self.connected = false;
                        }
                    }
                }
            }
        }

        if new_data {
            self.try_read_messages();
        }

        if self.connected == false && self.stream.is_some() {
            self.stream = None;
        }
    }

    fn bytes_to_string(bytes: &[u8]) -> String {
        bytes.iter().map(|&x| x as char).collect::<String>()
    }

    fn try_read_messages(&mut self) {
        loop {
            let line_break = self.buffer.find_first(&[13u8, 10u8]);
            match line_break {
                Some(up_to) => {
                    if let Some(message_bytes) = self.buffer.consume(up_to) {
                        self.buffer.discard(2); // Discard the line break
                        self.try_parse_message(Self::bytes_to_string(&message_bytes));
                    }
                }
                None => break
            }
        }
    }

    fn try_parse_message(&mut self, message: String) {
        if message.starts_with("PING") {
            let parts: Vec<&str> = message.split(' ').collect();
            if parts.len() == 2 {
                println!("<<< {message}");
                self.send_string(format!("PONG {}\r\n", parts[1]));
                return;
            }
        }

        println!("PARSE FAILED <<< {message}");
    }

    fn send_str(&mut self, command: &str) {
        print!(">>> {command}");
        self.send_bytes(command.as_bytes());
    }

    fn send_string(&mut self, command: String) {
        print!(">>> {command}");
        self.send_bytes(command.as_bytes());
    }

    #[inline]
    fn send_bytes(&mut self, command: &[u8]) {
        match self.stream {
            None => {}
            Some(ref mut stream) => {
                let _ = stream.write_all(command);
            }
        }
    }
}
