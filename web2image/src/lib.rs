mod args;
mod error;
mod screenshot;

pub use args::{get_args, Arg};
pub use error::AppError;
pub use screenshot::{generate_qr_code, save_screen_shot};
