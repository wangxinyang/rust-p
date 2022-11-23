mod abi;

pub use abi::*;
use bytes::{Bytes, BytesMut};
use prost::Message;

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

impl From<Response> for Bytes {
    fn from(value: Response) -> Self {
        let mut buf = BytesMut::with_capacity(value.encoded_len());
        value.encode(&mut buf).unwrap();
        buf.freeze()
    }
}
