mod pb;

use crate::pb::{Request, Response};
use anyhow::{Ok, Result};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let addr = "localhost:3001";
    let client = TcpStream::connect(addr).await?;
    let mut stream = LengthDelimitedCodec::builder()
        .length_field_length(2)
        .new_framed(client);
    let request = Request::new_get("hello".to_string());
    stream.send(request.into()).await?;

    let request = Request::new_put("hello".to_string(), "world".to_string());
    stream.send(request.into()).await?;

    while let Some(std::result::Result::Ok(buf)) = stream.next().await {
        let msg = Response::try_from(buf)?;
        println!("Got msg is: {:?}", msg);
    }

    Ok(())
}
