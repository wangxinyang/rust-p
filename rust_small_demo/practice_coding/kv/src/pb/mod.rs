mod abi;

pub use abi::*;
use bytes::BytesMut;
use prost::Message;

impl TryFrom<BytesMut> for Request {
    type Error = prost::DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(value)
    }
}
