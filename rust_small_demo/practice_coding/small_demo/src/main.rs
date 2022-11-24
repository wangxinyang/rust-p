use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use std::io;
use tokio::net::TcpListener;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub enum ConnectionError {
    Io(io::Error),
    Disconnected,
}

impl From<io::Error> for ConnectionError {
    fn from(err: io::Error) -> Self {
        ConnectionError::Io(err)
    }
}

type Message = String;

struct EchoCodec;

impl Encoder<Message> for EchoCodec {
    type Error = ConnectionError;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(item.as_bytes());
        Ok(())
    }
}

impl Decoder for EchoCodec {
    type Item = Message;

    type Error = ConnectionError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        let data = src.split_to(src.len());
        Ok(Some(String::from_utf8_lossy(&data[..]).into_owned()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:3300";
    let listener = TcpListener::bind(addr).await?;
    println!("listen on {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("accepted connection from: {}", addr);

        tokio::spawn(async move {
            let codec = EchoCodec {};
            let mut framed = codec.framed(stream);
            while let Some(message) = framed.next().await {
                if let Ok(message) = message {
                    println!("received message is: {:?}", message);
                    framed.send(message).await?;
                }
            }
            Ok::<(), ConnectionError>(())
        });
    }
}
