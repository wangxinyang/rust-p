use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Output path error")]
    PathError(String),
}
