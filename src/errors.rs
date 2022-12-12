// use clap::App;
use thiserror::Error;
use tokio::task::JoinError;

/// AppError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum AppError {
    /// Represents a failure to bind to a multicast address.
    #[error("big oops")]
    BindError(String),

    /// Represents a failure to join a multicast address.
    #[error("Unable to join multicast address")]
    JoinError(String),
}

impl std::convert::From<JoinError> for AppError {
    fn from(err: JoinError) -> Self {
        AppError::JoinError(err.to_string())
    }
}

impl std::convert::From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::BindError(err.to_string())
    }
}