use serde_json::Error as SerdeJsonError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    TokioRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    CommandError(#[from] io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error(transparent)]
    StringConvertUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
