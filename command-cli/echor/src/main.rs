use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "wangxinyang <wangxinyang1983@gmail.com>",
    version = "0.1.0",
    about = "Rust echo",
    long_about = None
)]
struct Args {
    /// TEXT
    #[clap(required = true, name = "TEXT", help = "Input text", min_values = 1)]
    text: Vec<String>,

    /// omit_newline
    #[clap(short, takes_value = false, help = "Do not print newline")]
    n: bool,
}

fn main() {
    let args = Args::parse();
    let text = args.text;
    print!("{}{}", text.join(" "), if args.n { "" } else { "\n" });
}

use assert_cmd::Command;
use predicates::prelude::predicate;
#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}
