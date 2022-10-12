mod api;

pub use api::*;
use chrono::Utc;

impl Token {
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }
}

impl LoginRequest {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            name: username.into(),
            pwd: password.into(),
        }
    }

    /// assumes get jwt token
    pub fn get_token(&self) -> Token {
        Token::new("Hello gRPC")
    }
}

impl SendMessageRequest {
    pub fn to_get_msg_res(&self, sender: impl Into<String>) -> GetMessageResponse {
        GetMessageResponse::new(sender, &self.room, &self.content)
    }
}

impl GetMessageResponse {
    pub fn new(
        sender: impl Into<String>,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            room: room.into(),
            content: content.into(),
            sender: sender.into(),
            timestamp: Utc::now().timestamp(),
        }
    }
}
