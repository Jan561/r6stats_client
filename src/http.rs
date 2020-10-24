pub mod error;
pub mod ratelimit;

pub use self::ratelimit::Ratelimit;

use self::error::{unsuccessful_request, url_error};
use crate::Error;
use reqwest::{Client, ClientBuilder, Method, Response, StatusCode, Url};
use std::fmt::{self, Debug, Formatter};

pub(crate) struct Http {
    client: Client,
    token: String,
    ratelimit: Ratelimit,
}

impl Http {
    pub fn new(token: &str, ratelimit: Ratelimit) -> Result<Self, Error> {
        let token = token.trim();
        let token = if token.starts_with("Bearer ") {
            token[7..].to_string()
        } else {
            token.to_string()
        };

        let client = ClientBuilder::new().use_rustls_tls().build()?;

        Ok(Self {
            client,
            token,
            ratelimit,
        })
    }

    pub fn ratelimit(&self) -> &Ratelimit {
        &self.ratelimit
    }

    pub async fn request(&mut self, path: &str) -> Result<Response, Error> {
        let url = Url::parse(path).map_err(|e| url_error(path, e))?;

        self.ratelimit.pre_hook(path).await?;

        let response = self
            .client
            .request(Method::GET, url)
            .bearer_auth(&self.token)
            .send()
            .await?;

        let status = response.status();
        if status != StatusCode::OK {
            return Err(unsuccessful_request(path, status));
        }

        Ok(response)
    }
}

impl Debug for Http {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Http")
            .field("client", &self.client)
            .field("ratelimit", &self.ratelimit)
            .finish()
    }
}
