use crate::Error as CrateError;
use reqwest::{Error as ReqwestError, StatusCode};
use std::time::Duration;
use url::ParseError;

#[derive(Debug)]
pub struct Error {
    pub url: Option<String>,
    pub kind: Kind,
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Self {
            url: e.url().map(|u| u.as_str().to_string()),
            kind: Kind::Request(e),
        }
    }
}

#[derive(Debug)]
pub enum Kind {
    UnsuccessfulRequest(StatusCode),
    UrlError(ParseError),
    PreRatelimited(Duration),
    Request(ReqwestError),
}

pub(crate) fn unsuccessful_request(url: &str, status: StatusCode) -> CrateError {
    CrateError::HttpError(Error {
        url: Some(url.to_string()),
        kind: Kind::UnsuccessfulRequest(status),
    })
}

pub(crate) fn url_error(url: &str, e: ParseError) -> CrateError {
    CrateError::HttpError(Error {
        url: Some(url.to_string()),
        kind: Kind::UrlError(e),
    })
}

pub(crate) fn ratelimited_error(url: &str, duration: Duration) -> CrateError {
    CrateError::HttpError(Error {
        url: Some(url.to_string()),
        kind: Kind::PreRatelimited(duration),
    })
}

pub(crate) fn request_error(e: ReqwestError) -> CrateError {
    CrateError::HttpError(Error {
        url: e.url().map(|u| u.as_str().to_string()),
        kind: Kind::Request(e),
    })
}
