#[macro_use]
mod macros;

pub mod error;
mod http;
pub mod platform;
pub mod regions;
pub mod stats;

use crate::error::Error;
use crate::http::error::{Error as HttpError, Kind as ErrorKind};
use crate::platform::Platform;
use crate::stats::generic::GenericStats;
use crate::stats::http::RouteInfo as StatsInfo;
use crate::stats::seasonal::SeasonalStats;
use crate::stats::Kind;
use reqwest::{Client as ReqwestClient, ClientBuilder, Method, Response, Url};

pub struct Client {
    client: ReqwestClient,
    token: String,
}

impl Client {
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

    pub async fn generic_stats(
        &self,
        username: &str,
        platform: Platform,
    ) -> Result<GenericStats, Error> {
        let response = self
            .stats_request(username, platform, Kind::Generic)
            .await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn seasonal_stats(
        &self,
        username: &str,
        platform: Platform,
    ) -> Result<SeasonalStats, Error> {
        let response = self
            .stats_request(username, platform, Kind::Seasonal)
            .await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn stats_request(
        &self,
        username: &str,
        platform: Platform,
        kind: Kind,
    ) -> Result<Response, Error> {
        let path = StatsInfo {
            username: username.to_string(),
            platform,
            kind,
        }
        .path();
        self.request(&path).await
    }

    async fn request(&self, path: &str) -> Result<Response, Error> {
        let url = Url::parse(path).map_err(|e| {
            Error::HttpError(HttpError {
                url: Some(path.to_string()),
                kind: ErrorKind::UrlError(e),
            })
        })?;
        self.client
            .request(Method::GET, url)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
#[tokio::test]
async fn test() {
    let token = std::env::var("R6STATS_TOKEN").expect("Cannot find token in env.");
    let client = Client::new(&token).unwrap();
    let _ = client
        .generic_stats("pengu.g2", Platform::Pc)
        .await
        .unwrap();
}
