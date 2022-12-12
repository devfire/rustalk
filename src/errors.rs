// use clap::App;
use thiserror::Error;

/// AppError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum AppError {
    /// Represents a failure to bind to a multicast address.
    #[error("big oops")]
    BindError(String),
}

impl std::convert::From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::BindError(err.to_string())
    }
}