pub mod error;
pub(crate) mod ratelimit;

pub use self::ratelimit::Ratelimit;

use self::error::{unsuccessful_request, url_error};
use crate::Error;
use reqwest::{Client, ClientBuilder, Method, Response, StatusCode, Url};

pub(crate) struct Http {
    client: Client,
    token: String,
    ratelimit: Ratelimit,
}

impl Http {
    pub fn new(token: &str, ratelimit: Option<u16>) -> Result<Self, Error> {
        let token = token.trim();
        let token = if token.starts_with("Bearer ") {
            token[7..].to_string()
        } else {
            token.to_string()
        };

        let client = ClientBuilder::new().use_rustls_tls().build()?;

        let ratelimit = match ratelimit {
            Some(limit) => Ratelimit::with_limit(limit),
            None => Ratelimit::new(),
        };

        Ok(Self {
            client,
            token,
            ratelimit,
        })
    }

    pub async fn request(&mut self, path: &str) -> Result<Response, Error> {
        let url = Url::parse(path).map_err(|e| Error::HttpError(url_error(path, e)))?;

        self.ratelimit.pre_hook().await;

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
