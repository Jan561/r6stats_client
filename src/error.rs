use crate::http::error::{request_error, Error as HttpError};
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
        Self::HttpError(request_error(e))
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Self::JsonError(e)
    }
}
