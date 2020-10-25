//! Module containing the client.

use super::{LeaderboardClient, StatsClient};
use crate::http::{Ratelimit, RatelimitBuilder};
use crate::{Error, Http, Pointer};

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
        let http = Pointer::new(Http::new(token, ratelimit)?);

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
        self.http.ratelimit().await
    }
}
