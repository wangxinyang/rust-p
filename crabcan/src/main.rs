use crate::errors::exit_with_retcode;
use tracing::{error, info};

mod cli;
mod config;
mod container;
mod errors;

#[macro_use]
extern crate scan_fmt;
fn main() {
    match cli::parse_args() {
        Ok(args) => {
            info!("{:?}", args);
            exit_with_retcode(container::start(args));
        }
        Err(e) => {
            let retcode = e.get_retcode();
            error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            std::process::exit(retcode);
        }
    };
}
