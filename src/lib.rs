#[macro_use]
mod macros;

pub mod error;
pub mod http;
pub mod leaderboard;
pub mod platform;
pub mod region;
pub mod stats;

pub(crate) mod pointer;

pub use crate::error::Error;
pub use crate::platform::Platform;
pub use crate::region::Region;

pub(crate) use crate::http::Http;
pub(crate) use crate::pointer::Pointer;

use crate::leaderboard::Client as LeaderboardClient;
use crate::stats::Client as StatsClient;

pub struct Client {
    stats: StatsClient,
    leaderboard: LeaderboardClient,
}

impl Client {
    pub fn new(token: &str) -> Result<Self, Error> {
        let http = new_ptr!(Http::new(token, None)?);

        let stats = StatsClient::new(http.clone());
        let leaderboard = LeaderboardClient::new(http);

        let s = Self { stats, leaderboard };
        Ok(s)
    }

    pub fn stats(&self) -> &StatsClient {
        &self.stats
    }

    pub fn leaderboard(&self) -> &LeaderboardClient {
        &self.leaderboard
    }
}

#[cfg(test)]
#[tokio::test]
async fn test() {
    let token = std::env::var("R6STATS_TOKEN").expect("Cannot find token in env.");
    let client = Client::new(&token).unwrap();
    for _ in 0..60 {
        let _ = client.stats().generic("pengu.g2", Platform::Pc).await;
    }

    let response = client.stats().generic("pengu.g2", Platform::Pc).await;

    println!("{:?}", response);
}
