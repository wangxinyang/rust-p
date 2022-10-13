use std::{
    env,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use tonicr::client::Client;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let username = env::var("NAME")?;
    info!("Client new: {:?}", username);
    let mut client = Client::new(&username).await;
    // login
    client.login().await?;

    // get messages
    client.get_message(&username).await?;

    // send message
    let mut stdin = BufReader::new(std::io::stdin()).lines();
    while let Some(Ok(content)) = stdin.next() {
        if content == ":q" {
            break;
        }
        client.send_messaeg("test", content).await?
    }
    Ok(())
}
