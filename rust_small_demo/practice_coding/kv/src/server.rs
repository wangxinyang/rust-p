mod pb;

use std::sync::Arc;

use pb::*;

use crate::pb::request::Command;
use anyhow::Result;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;

#[derive(Debug)]
struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        //Setting a filter based on the value of the RUST_LOG environment variable: RUST_LOG=info cargo run --bin server
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = Arc::new(ServerState::new());
    let addr = "0.0.0.0:3001";
    let listener = TcpListener::bind(addr).await?;
    info!("Listen on {}", addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("Accepted connection from {}", addr);
        let shared = state.clone();
        tokio::spawn(async move {
            let mut stream = LengthDelimitedCodec::builder()
                .length_field_length(2)
                .new_framed(socket);
            while let Some(std::result::Result::Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?;
                info!("Accepted msg is: {:?}", msg);

                let response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => match shared.store.get(&key) {
                        Some(v) => Response::new(key, String::from_utf8(v.value().to_vec())?),
                        None => Response::not_found(key),
                    },
                    Some(Command::Put(RequestPut { key, value })) => {
                        shared.store.insert(key.clone(), value.clone().into_bytes());
                        Response::new(key, value)
                    }
                    None => unimplemented!("Not implemented"),
                };
                stream.send(response.into()).await?;
            }
            std::result::Result::Ok::<(), anyhow::Error>(())
        });
    }
}
