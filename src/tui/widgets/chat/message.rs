use crate::client::UserInfo;

#[derive(Clone)]
pub enum Message {
    FromUser { user: UserInfo, text: String },
    Join { user: UserInfo },
    Leave { user: UserInfo, reason: String },
    Quick { user: UserInfo, kicked_by: UserInfo, reason: String },
    ChangeDay { date: time::Date },
    Mode { user: UserInfo, mode: String, changed_by: UserInfo },
    Info { message: String },
}

impl From<&str> for Message {
    fn from(value: &str) -> Self {
        Message::Info { message: value.to_string() }
    }
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Message::Info { message: value.to_string() }
    }
}
