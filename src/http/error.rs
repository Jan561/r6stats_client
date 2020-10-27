use crate::Error as CrateError;
use reqwest::{Error as ReqwestError, StatusCode};
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use url::ParseError;

/// Errors related to HTTP.
#[derive(Debug)]
pub struct Error {
    url: Option<String>,
    kind: Kind,
}

impl Error {
    /// Returns the URL associated to the error, if available.
    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    /// Returns the status code, if the error was generated from a response.
    pub fn unsuccessful_request(&self) -> Option<StatusCode> {
        if let Kind::UnsuccessfulRequest(status) = self.kind {
            Some(status)
        } else {
            None
        }
    }

    /// Returns the parse error, if the error was generated from URL parsing.
    pub fn url_error(&self) -> Option<ParseError> {
        if let Kind::UrlError(err) = self.kind {
            Some(err)
        } else {
            None
        }
    }

    /// Returns the request error, if the error was generated from a request.
    pub fn request_error(&self) -> Option<&ReqwestError> {
        if let Kind::RequestError(ref err) = self.kind {
            Some(err)
        } else {
            None
        }
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Self {
            url: e.url().map(|u| u.as_str().to_string()),
            kind: Kind::RequestError(e),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.kind {
            Kind::UnsuccessfulRequest(_) => None,
            Kind::UrlError(ref err) => Some(err),
            Kind::RequestError(ref err) => Some(err),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.kind, f)
    }
}

#[derive(Debug)]
enum Kind {
    UnsuccessfulRequest(StatusCode),
    UrlError(ParseError),
    RequestError(ReqwestError),
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Self::UnsuccessfulRequest(status) => write!(f, "Unsuccessful request: {}", status),
            Self::UrlError(ref err) => write!(f, "Url error: {}", err),
            Self::RequestError(ref err) => write!(f, "Request Error: {}", err),
        }
    }
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

pub(crate) fn request_error(e: ReqwestError) -> CrateError {
    CrateError::HttpError(Error {
        url: e.url().map(|u| u.as_str().to_string()),
        kind: Kind::RequestError(e),
    })
}
