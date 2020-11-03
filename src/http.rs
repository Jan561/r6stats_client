//! HTTP module which provides functions for performing requests to the R6Stats endpoints.

pub(crate) mod error;

#[cfg(feature = "ratelimiting")]
mod ratelimit;

pub use self::error::Error;
#[cfg(feature = "ratelimiting")]
pub use self::ratelimit::{Ratelimit, RatelimitBuilder};

use self::error::{unsuccessful_request, url_error};
#[cfg(feature = "ratelimiting")]
use crate::internals::Cell;
use crate::Error as CrateError;
use reqwest::{Client, ClientBuilder, Method, Response, StatusCode, Url};
use std::fmt::{self, Debug, Formatter};

pub(crate) struct Http {
    client: Client,
    token: String,
    #[cfg(feature = "ratelimiting")]
    ratelimit: Cell<Ratelimit>,
}

impl Http {
    pub fn new(
        token: &str,
        #[cfg(feature = "ratelimiting")] ratelimit: Ratelimit,
    ) -> Result<Self, CrateError> {
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
            #[cfg(feature = "ratelimiting")]
            ratelimit: Cell::new(ratelimit),
        })
    }

    #[cfg(feature = "ratelimiting")]
    pub async fn ratelimit(&self) -> Ratelimit {
        *borrow!(self.ratelimit)
    }

    pub async fn request(&self, path: &str) -> Result<Response, CrateError> {
        let url = Url::parse(path).map_err(|e| url_error(path, e))?;

        #[cfg(feature = "ratelimiting")]
        borrow_mut!(self.ratelimit).pre_hook().await?;

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
        let mut d = f.debug_struct("Http");

        d.field("client", &self.client);

        #[cfg(feature = "ratelimiting")]
        d.field("ratelimit", &self.ratelimit);

        d.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{Http, Ratelimit};

    #[test]
    fn test_token_formatting() {
        let expected = "ABC";
        let tokens = ["ABC", "  ABC  ", "Bearer ABC", "  Bearer ABC "];

        for &token in tokens.iter() {
            let http = Http::new(token, Ratelimit::default()).unwrap();
            assert_eq!(http.token, expected);
        }
    }

    #[tokio::test]
    async fn test_request() {
        let http = Http::new("", Ratelimit::default()).unwrap();

        let _ = http
            .request("https://httpbin.org/status/200")
            .await
            .unwrap();
        let _ = http
            .request("https://httpbin.org/status/404")
            .await
            .unwrap_err();
    }
}
