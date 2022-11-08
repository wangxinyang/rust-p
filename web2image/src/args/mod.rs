use crate::AppError;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version = "1.0", author, about, long_about = None)]
pub struct Arg {
    #[arg(short, long)]
    pub url: String,

    #[arg(short, long, default_value = "./")]
    pub output: PathBuf,
}

pub fn get_args() -> Result<Arg, AppError> {
    let args = Arg::parse();
    let output_path = &args.output;
    // the path is not exists or not a dir
    if !output_path.exists() || !output_path.is_dir() {
        return Err(AppError::PathError(
            output_path.to_str().unwrap().to_string(),
        ));
    }
    Ok(args)
}
