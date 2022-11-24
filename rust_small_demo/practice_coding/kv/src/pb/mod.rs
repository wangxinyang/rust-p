mod abi;

use self::request::Command;
pub use abi::*;
use bytes::{Bytes, BytesMut};
use prost::Message;

impl Request {
    pub fn new_get(key: String) -> Self {
        Self {
            command: Some(Command::Get(RequestGet { key })),
        }
    }

    pub fn new_put(key: String, value: String) -> Self {
        Self {
            command: Some(Command::Put(RequestPut { key, value })),
        }
    }
}

impl Response {
    pub fn new(key: String, value: String) -> Self {
        Self {
            code: 0,
            key,
            value,
        }
    }

    pub fn not_found(key: String) -> Self {
        Self {
            code: 404,
            key,
            value: Default::default(),
        }
    }
}

impl TryFrom<BytesMut> for Request {
    type Error = prost::DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(value)
    }
}

impl TryFrom<BytesMut> for Response {
    type Error = prost::DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(value)
    }
}

impl From<Request> for Bytes {
    fn from(value: Request) -> Self {
        let mut buf = BytesMut::with_capacity(value.encoded_len());
        value.encode(&mut buf).unwrap();
        buf.freeze()
    }
}

impl From<Response> for Bytes {
    fn from(value: Response) -> Self {
        let mut buf = BytesMut::with_capacity(value.encoded_len());
        value.encode(&mut buf).unwrap();
        buf.freeze()
    }
}
