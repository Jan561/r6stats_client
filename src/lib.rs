#[macro_use]
mod macros;

pub mod error;
pub mod http;
pub mod platform;
pub mod region;
pub mod stats;

pub use crate::error::Error;
pub use crate::platform::Platform;

use crate::http::Http;
use crate::stats::Client as StatsClient;
use std::rc::Rc;

pub struct Client {
    stats: StatsClient,
}

impl Client {
    pub fn new(token: &str) -> Result<Self, Error> {
        let http = Rc::new(Http::new(token)?);

        let stats = StatsClient::new(http.clone());

        let s = Self { stats };
        Ok(s)
    }

    pub fn stats(&self) -> &StatsClient {
        &self.stats
    }
}

#[cfg(test)]
#[tokio::test]
async fn test() {
    let token = std::env::var("R6STATS_TOKEN").expect("Cannot find token in env.");
    let client = Client::new(&token).unwrap();
    let _ = client
        .stats()
        .weapons("pengu.g2", Platform::Pc)
        .await
        .unwrap();
}
