//! Module containing crate errors.

use crate::http::error::{request_error, Error as HttpError};
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

/// Enum containing all errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    HttpError(HttpError),
    JsonError(JsonError),
    Other(String),
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        request_error(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Self::JsonError(e)
    }
}
