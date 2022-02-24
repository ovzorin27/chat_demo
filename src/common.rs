use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub(crate) enum MessageType {
    Message,
    Connected,
    Disconnected,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Message {
    pub(crate) who: String,
    pub(crate) msg_type: MessageType,
    pub(crate) payload: Option<String>
}

impl Message {
    pub(crate) fn handshake(from: String) -> Message {
        Message {
            who: from,
            msg_type: MessageType::Connected,
            payload: None
        }
    }

    pub(crate) fn simple(from: String, message: String) -> Message {
        Message {
            who: from,
            msg_type: MessageType::Message,
            payload: Some(message)
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.msg_type {
            MessageType::Message => write!(f, "{}: {}", self.who, self.payload.as_ref().unwrap()),
            MessageType::Connected => write!(f, "* {} connected *", self.who),
            MessageType::Disconnected => write!(f, "* {} disconnected *", self.who)
        }
    }
}