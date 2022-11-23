mod abi;

pub use abi::*;
use prost::bytes::BytesMut;

use self::request::Message;

impl TryFrom<BytesMut> for Request {
    type Error = prost::DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        // value.
        // let request_message: Message = prost::Message::decode(value)?;
        // Ok(Request {
        //     message: Some(match request_message {
        //         RequestGet { key } => request::Message::Get(RequestGet { key }),
        //         RequestPut { key, value } => request::Message::Put(RequestPut { key, value }),
        //     }),
        // })
        todo!()
    }
}
