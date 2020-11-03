//! Module for ratelimiting requests before sending them.

use crate::Error;
use tokio::time::{delay_for, Duration, Instant};

const DEFAULT_RATE_LIMIT: u16 = 60;
const DEFAULT_INTERVAL: Duration = Duration::from_secs(60);

/// The builder for [`Ratelimit`].
///
/// [`Ratelimit`]: struct.Ratelimit.html
#[derive(Copy, Clone, Debug, Default)]
pub struct RatelimitBuilder {
    limit: Option<u16>,
    interval: Option<Duration>,
}

impl RatelimitBuilder {
    /// Returns a new builder.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Sets the [`limit`] of the [`Ratelimit`].
    ///
    /// Set to `0` to disable ratelimiting.
    ///
    /// [`limit`]: struct.Ratelimit.html#method.limit
    /// [`Ratelimit`]: struct.Ratelimit.html
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the [`interval`] of the [`Ratelimit`].
    ///
    /// [`interval`]: struct.Ratelimit.html#method.interval
    /// [`Ratelimit`]: struct.Ratelimit.html.
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Returns the finished [`Ratelimit`].
    ///
    /// [`Ratelimit`]: struct.Ratelimit.html
    pub(crate) fn build(self) -> Ratelimit {
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
#[derive(Copy, Clone, Debug)]
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

        let delay = match self.reset_in() {
            Some(delay) => delay,
            None => {
                self.reset();
                self.remaining -= 1;

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

    fn reset(&mut self) {
        self.reset_at = Instant::now() + self.interval;
        self.remaining = self.limit;
    }
}

impl Default for Ratelimit {
    fn default() -> Self {
        RatelimitBuilder::new().build()
    }
}

#[cfg(test)]
mod tests {
    use crate::http::{Ratelimit, RatelimitBuilder};
    use tokio::time::{Duration, Instant};

    const NO_LIMIT_TEST_CAP: usize = 100;

    #[tokio::test]
    async fn test_default_ratelimit() {
        let r = Ratelimit::default();

        assert_eq!(r.limit(), 60);
        assert_eq!(r.remaining(), 60);
        assert_eq!(r.interval(), Duration::from_secs(60));
        assert!(r.reset_in().is_none());
    }

    #[tokio::test]
    async fn test_ratelimiting() {
        // Limit of 3 requests in 3 seconds
        let mut r = RatelimitBuilder::new()
            .limit(3)
            .interval(Duration::from_secs(3))
            .build();

        // Initial reset
        r.reset();

        // Save reset_at to assert that following requests pass
        let reset_at = r.reset_at();

        for i in 0..r.limit() {
            r.pre_hook().await.unwrap();
            assert_eq!(r.remaining(), r.limit() - i - 1);
        }

        // Check if wrong ratelimited
        assert_eq!(reset_at, r.reset_at());

        // Next request should get ratelimited
        r.pre_hook().await.unwrap();

        assert!(Instant::now().checked_duration_since(reset_at).is_some());

        r.pre_hook().await.unwrap();

        assert_eq!(r.remaining(), r.limit() - 1);
        assert!(r.reset_in().is_some());
    }

    #[tokio::test]
    async fn test_disabled_ratelimiting() {
        let mut r = RatelimitBuilder::new().limit(0).build();

        r.reset();

        let reset_at = r.reset_at();

        for _ in 0..NO_LIMIT_TEST_CAP {
            r.pre_hook().await.unwrap();
        }

        assert_eq!(reset_at, r.reset_at());
    }
}
