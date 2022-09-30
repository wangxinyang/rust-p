mod command;
mod request;

use anyhow::{Ok, Result};
use request::{get_response, Config};
use similar::{ChangeTag, TextDiff};

#[tokio::main]
async fn main() -> Result<()> {
    // get config
    let config = Config::new("yaml/request.yml")?;

    // get diff contents
    let (old, new) = get_response(config.requests).await?;

    // diff response
    let diff = TextDiff::from_lines(&old, &new);
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{}{}", sign, change);
    }

    Ok(())
}
