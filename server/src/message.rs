use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum MessageParseError {
    #[error("Got invalid cmd {0}")]
    InvalidCmd(String),
    #[error("Missing part from message")]
    Incomplete,
}

#[derive(Debug, Clone)]
pub enum Message {
    SendMsg { user: String, text: String },
    Error(String),
    System(String),
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::SendMsg { user, text } => write!(f, "MSG {user} {text}"),
            Message::Error(error_msg) => write!(f, "ERROR {error_msg}"),
            Message::System(s) => write!(f, "SYSTEM {}", s),
        }
    }
}

impl From<MessageParseError> for Message {
    fn from(value: MessageParseError) -> Self {
        Message::Error(value.to_string())
    }
}

impl FromStr for Message {
    type Err = MessageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, remainder) = s.split_once(' ').ok_or(MessageParseError::Incomplete)?;

        match cmd {
            "MSG" => {
                let mut parts = remainder.splitn(2, ' ');
                let user = parts
                    .next()
                    .ok_or(MessageParseError::Incomplete)?
                    .to_string();

                let text = parts
                    .next()
                    .ok_or(MessageParseError::Incomplete)?
                    .to_string();

                Ok(Message::SendMsg { user, text })
            }
            _ => Err(MessageParseError::InvalidCmd(cmd.to_string())),
        }
    }
}
