//! Module for ratelimiting requests before sending them.

use crate::Error;
use tokio::time::{delay_until, Duration, Instant};

const DEFAULT_RATE_LIMIT: u16 = 60;
const DEFAULT_INTERVAL: Duration = Duration::from_secs(60);

/// The builder for [`Ratelimit`].
///
/// [`Ratelimit`]: struct.Ratelimit.html
pub struct RatelimitBuilder {
    limit: Option<u16>,
    interval: Option<Duration>,
}

impl RatelimitBuilder {
    /// Returns a new builder.
    pub fn new() -> Self {
        Self {
            limit: None,
            interval: None,
        }
    }

    /// Sets the [`limit`] of the [`Ratelimit`].
    ///
    /// Set to `0` to disable ratelimiting.
    ///
    /// [`limit`]: struct.Ratelimit.html#method.limit
    /// [`Ratelimit`]: struct.Ratelimit.html
    pub fn limit(&mut self, limit: u16) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the [`interval`] of the [`Ratelimit`].
    ///
    /// [`interval`]: struct.Ratelimit.html#method.interval
    /// [`Ratelimit`]: struct.Ratelimit.html.
    pub fn interval(&mut self, interval: Duration) -> &mut Self {
        self.interval = Some(interval);
        self
    }

    /// Returns the finished [`Ratelimit`].
    ///
    /// [`Ratelimit`]: struct.Ratelimit.html
    pub fn build(self) -> Ratelimit {
        let limit = self.limit.unwrap_or(DEFAULT_RATE_LIMIT);
        Ratelimit {
            limit,
            remaining: limit,
            interval: self.interval.unwrap_or(DEFAULT_INTERVAL),
            reset_at: Instant::now(),
        }
    }
}

/// Handles ratelimiting.
///
/// When a request exceeds the limit, it will be delayed until next [`reset`].
///
/// [`reset`]: struct.Ratelimit.html#method.reset_at
#[derive(Clone, Debug)]
pub struct Ratelimit {
    limit: u16,
    remaining: u16,
    interval: Duration,
    reset_at: Instant,
}

impl Ratelimit {
    /// The total number of requests that can be made within the [`interval`].
    ///
    /// Returns `0` if not ratelimited.
    ///
    /// [`interval`]: struct.Ratelimit.html#method.interval
    pub fn limit(&self) -> u16 {
        self.limit
    }

    /// The number of requests remaining in the period of time.
    pub fn remaining(&self) -> u16 {
        self.remaining
    }

    /// The ratelimit interval.
    pub fn interval(&self) -> Duration {
        self.interval
    }

    /// The absolute time at which the ratelimit resets.
    pub fn reset_at(&self) -> Instant {
        self.reset_at
    }

    /// The duration from now in which the ratelimit resets.
    ///
    /// Returns `None` if the reset time is in the past.
    pub fn reset_in(&self) -> Option<Duration> {
        self.reset_at.checked_duration_since(Instant::now())
    }

    pub(super) async fn pre_hook(&mut self) -> Result<(), Error> {
        if self.limit == 0 {
            return Ok(());
        }

        if Instant::now() > self.reset_at {
            self.remaining = self.limit - 1;
            return Ok(());
        }

        if self.remaining == 0 {
            delay_until(self.reset_at).await;
            return Ok(());
        }

        self.remaining -= 1;

        Ok(())
    }
}

impl Default for Ratelimit {
    fn default() -> Self {
        RatelimitBuilder::new().build()
    }
}
