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

use crate::http::ratelimit::RatelimitBuilder;
use crate::http::Ratelimit;
use crate::leaderboard::Client as LeaderboardClient;
use crate::stats::Client as StatsClient;

/// Client for the R6Stats-API.
///
/// Use this client to send authenticated requests to the r6stats-api
#[derive(Clone, Debug)]
pub struct Client {
    http: Pointer<Http>,
    stats: StatsClient,
    leaderboard: LeaderboardClient,
}

impl Client {
    fn _new(token: &str, ratelimit: Ratelimit) -> Result<Self, Error> {
        let http = new_ptr!(Http::new(token, ratelimit)?);

        let stats = StatsClient::new(http.clone());
        let leaderboard = LeaderboardClient::new(http.clone());

        let s = Self {
            stats,
            leaderboard,
            http,
        };
        Ok(s)
    }

    /// Creates a new client with default ratelimiting.
    ///
    /// # Args
    ///
    /// - `token` - The API key for authentication with the endpoint
    pub fn new(token: &str) -> Result<Self, Error> {
        Self::_new(token, Ratelimit::default())
    }

    /// Creates a new client with custom ratelimit settings.
    ///
    /// # Args
    ///
    /// - `token` - The API key for authentication with the endpoint
    /// - `op` - Closure for building the [`Ratelimit`]
    ///
    /// [`Ratelimit`]: http/ratelimit/struct.Ratelimit.html
    pub fn with_ratelimit<F>(token: &str, op: F) -> Result<Self, Error>
    where
        F: FnOnce(&mut RatelimitBuilder) -> &mut RatelimitBuilder,
    {
        let mut builder = RatelimitBuilder::new();
        op(&mut builder);
        let ratelimit = builder.build();

        Self::_new(token, ratelimit)
    }

    /// Returns the client for requests to the stats endpoint.
    pub fn stats(&self) -> &StatsClient {
        &self.stats
    }

    /// Returns the client for requests to the leaderboard endpoint.
    pub fn leaderboard(&self) -> &LeaderboardClient {
        &self.leaderboard
    }

    /// Returns the current ratelimit.
    pub async fn ratelimit(&self) -> Ratelimit {
        deref!(self.http).ratelimit().clone()
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
