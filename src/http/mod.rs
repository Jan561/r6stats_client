pub mod error;

use self::error::{unsuccessful_request, url_error};
use crate::Error;
use reqwest::{Client, ClientBuilder, Method, Response, StatusCode, Url};

pub(crate) struct Http {
    client: Client,
    token: String,
}

impl Http {
    pub fn new(token: &str) -> Result<Self, Error> {
        let token = token.trim();
        let token = if token.starts_with("Bearer ") {
            token[7..].to_string()
        } else {
            token.to_string()
        };

        let client = ClientBuilder::new().use_rustls_tls().build()?;

        Ok(Self { client, token })
    }

    pub async fn request(&self, path: &str) -> Result<Response, Error> {
        let url = Url::parse(path).map_err(|e| Error::HttpError(url_error(path, e)))?;
        let response = self
            .client
            .request(Method::GET, url)
            .bearer_auth(&self.token)
            .send()
            .await?;

        let status = response.status();
        if status != StatusCode::OK {
            return Err(Error::HttpError(unsuccessful_request(path, status)));
        }

        Ok(response)
    }
}
