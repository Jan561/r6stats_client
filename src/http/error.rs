use crate::http::Status;
use reqwest::Error as ReqwestError;
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
    UnsuccessfulRequest(Status),
    UrlError(ParseError),
    Request(ReqwestError),
}
