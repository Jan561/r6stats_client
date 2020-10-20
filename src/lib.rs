#[macro_use]
mod macros;

mod date_format;
pub mod error;
pub mod platform;
pub mod routing;
pub mod stats;

use crate::platform::Platform;
use crate::routing::{Route, StatsInfo};
use crate::stats::{Kind, SeasonalStats};
use reqwest::{Client as ReqwestClient, ClientBuilder, Method, Response, Url};
use std::env;

pub struct Client {
    client: ReqwestClient,
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Self {
        let token = token.trim();
        let token = if token.starts_with("Bearer ") {
            token[7..].to_string()
        } else {
            token.to_string()
        };

        let client = ClientBuilder::new()
            .use_rustls_tls()
            .build()
            .expect("Cannot create client.");

        Self { client, token }
    }

    pub async fn stats_seasonal(&self, username: &str, platform: Platform) {
        let response = self.stats_request(username, platform, Kind::Seasonal).await;
        let stats: SeasonalStats = response.json().await.unwrap();
        println!("{:#?}", stats);
    }

    async fn stats_request(&self, username: &str, platform: Platform, kind: Kind) -> Response {
        let route = Route::Stats(StatsInfo {
            username: username.to_string(),
            platform,
            kind,
        });
        let url = Url::parse(&route.path()).unwrap();
        let request = self
            .client
            .request(Method::GET, url)
            .bearer_auth(&self.token)
            .build()
            .unwrap();
        self.client.execute(request).await.unwrap()
    }
}

#[tokio::test]
async fn test() {
    let token = env::var("R6STATS_TOKEN").expect("Cannot find token in env.");
    let client = Client::new(&token);
    client.stats_seasonal("Object.L1E", Platform::Pc).await;
}
