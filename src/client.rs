//! Module containing the client.

#[cfg(feature = "ratelimiting")]
use crate::http::{Ratelimit, RatelimitBuilder};
use crate::internals::Rc;
use crate::leaderboard::Client as LeaderboardClient;
use crate::stats::Client as StatsClient;
use crate::{Error, Http};

/// Client for the r6stats API.
///
/// Use this client to send authenticated requests to the r6stats-api.
///
/// # Ratelimiting
///
/// The API is (normally) ratelimited to 60 requests per minute. To prevent HTTP 429 errors,
/// ratelimits are enforced prior to sending requests by [`Ratelimit`].
///
/// [`Ratelimit`]: http/ratelimit/struct.Ratelimit.html
#[derive(Clone, Debug)]
pub struct Client {
    http: Rc<Http>,
    stats: StatsClient,
    leaderboard: LeaderboardClient,
}

impl Client {
    fn _new(
        token: &str,
        #[cfg(feature = "ratelimiting")] ratelimit: Ratelimit,
    ) -> Result<Self, Error> {
        let http = Rc::new(Http::new(
            token,
            #[cfg(feature = "ratelimiting")]
            ratelimit,
        )?);

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
    pub fn new(token: impl AsRef<str>) -> Result<Self, Error> {
        Self::_new(
            token.as_ref(),
            #[cfg(feature = "ratelimiting")]
            Ratelimit::default(),
        )
    }

    /// Creates a new client with custom ratelimit settings.
    ///
    /// # Args
    ///
    /// - `token` - The API key for authentication with the endpoint
    /// - `op` - Closure for building the [`Ratelimit`]
    ///
    /// [`Ratelimit`]: http/ratelimit/struct.Ratelimit.html
    #[cfg(feature = "ratelimiting")]
    pub fn with_ratelimit<F>(token: impl AsRef<str>, op: F) -> Result<Self, Error>
    where
        F: FnOnce(RatelimitBuilder) -> RatelimitBuilder,
    {
        let builder = op(RatelimitBuilder::new());
        let ratelimit = builder.build();

        Self::_new(token.as_ref(), ratelimit)
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
    #[cfg(feature = "ratelimiting")]
    pub async fn ratelimit(&self) -> Ratelimit {
        self.http.ratelimit().await
    }
}
