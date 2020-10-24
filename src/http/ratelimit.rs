use crate::Error;
use std::time::{Duration, SystemTime};
use tokio::time::delay_for;

const DEFAULT_RATE_LIMIT: u16 = 60;
const DEFAULT_INTERVAL: Duration = Duration::from_secs(60);

pub struct RatelimitBuilder {
    limit: Option<u16>,
    interval: Option<Duration>,
}

impl RatelimitBuilder {
    pub fn new() -> Self {
        Self {
            limit: None,
            interval: None,
        }
    }

    pub fn limit(&mut self, limit: u16) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn interval(&mut self, interval: Duration) -> &mut Self {
        self.interval = Some(interval);
        self
    }

    pub fn build(self) -> Ratelimit {
        let limit = self.limit.unwrap_or(DEFAULT_RATE_LIMIT);
        Ratelimit {
            limit,
            remaining: limit,
            interval: self.interval.unwrap_or(DEFAULT_INTERVAL),
            reset_at: SystemTime::now(),
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
    reset_at: SystemTime,
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

    pub fn interval(&self) -> Duration {
        self.interval
    }

    /// The absolute time at which the ratelimit resets.
    pub fn reset_at(&self) -> SystemTime {
        self.reset_at
    }

    /// The duration from now in which the ratelimit resets.
    ///
    /// Returns `None` if the reset time is in the past.
    pub fn reset_in(&self) -> Option<Duration> {
        self.reset_at.duration_since(SystemTime::now()).ok()
    }

    pub(super) async fn pre_hook(&mut self) -> Result<(), Error> {
        if self.limit == 0 {
            return Ok(());
        }

        let delay = match self.reset_in() {
            Some(delay) => delay,
            None => {
                // Ratelimit reset time in the past
                self.remaining = self.limit - 1;
                self.reset_at = SystemTime::now() + self.interval;

                return Ok(());
            }
        };

        if self.remaining == 0 {
            delay_for(delay).await;
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
