//! Module containing crate errors.

use crate::http::error::{request_error, Error as HttpError};
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
/// Enum containing all errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    HttpError(HttpError),
    JsonError(JsonError),
    UsernameMalformed,
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

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::HttpError(err) => Some(err),
            Self::JsonError(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::HttpError(err) => Display::fmt(err, f),
            Self::JsonError(err) => Display::fmt(err, f),
            Self::UsernameMalformed => write!(f, "Username is malformed."),
            Self::Other(s) => Display::fmt(s, f),
        }
    }
}
