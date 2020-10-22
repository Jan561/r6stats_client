use crate::http::error::{Error as HttpError, Kind};
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    HttpError(HttpError),
    JsonError(JsonError),
    Other(String),
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Self::HttpError(HttpError {
            url: e.url().map(|u| u.as_str().to_string()),
            kind: Kind::Request(e),
        })
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Self::JsonError(e)
    }
}
