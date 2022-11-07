use std::time::{SystemTime, UNIX_EPOCH};

pub struct Msg {
    pub id: usize,
    pub time: u64,
    pub name: String,
    pub data: MsgData,
}

pub enum MsgData {
    Login,
    Leave,
    Message(String),
}

impl Msg {
    pub fn new(id: usize, name: String, data: MsgData) -> Self {
        Msg {
            id,
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            name,
            data,
        }
    }

    pub fn login(id: usize, name: &str) -> Self {
        Msg::new(id, name.into(), MsgData::Login)
    }

    pub fn leave(id: usize, name: &str) -> Self {
        Msg::new(id, name.into(), MsgData::Leave)
    }

    pub fn message(id: usize, name: &str, msg: &str) -> Self {
        Msg::new(id, name.into(), MsgData::Message(msg.into()))
    }
}
