use std::time::SystemTime;
use tokio::time::{delay_for, Duration};

const DEFAULT_RATE_LIMIT: u16 = 60;
const RESET_AFTER: Duration = Duration::from_secs(60);

#[derive(Clone, Debug)]
pub struct Ratelimit {
    limit: u16,
    remaining: u16,
    reset_at: Option<SystemTime>,
}

impl Ratelimit {
    pub fn new() -> Self {
        Self::with_limit(DEFAULT_RATE_LIMIT)
    }

    pub fn with_limit(limit: u16) -> Self {
        Self {
            limit,
            remaining: limit,
            reset_at: None,
        }
    }

    /// The total number of requests that can be made within 60 seconds.
    ///
    /// `0` if not rate-limited
    pub fn limit(&self) -> u16 {
        self.limit
    }

    /// The number of requests remaining in the period of time.
    pub fn remaining(&self) -> u16 {
        self.remaining
    }

    pub fn reset_at(&self) -> Option<SystemTime> {
        self.reset_at
    }

    pub fn reset_in(&self) -> Option<Duration> {
        self.reset_at?.duration_since(SystemTime::now()).ok()
    }

    pub async fn pre_hook(&mut self) {
        if self.limit == 0 {
            return;
        }

        let delay = match self.reset_in() {
            Some(delay) => delay,
            None => {
                self.remaining = self.limit;
                self.reset_at = Some(SystemTime::now() + RESET_AFTER);

                return;
            }
        };

        if self.remaining == 0 {
            delay_for(delay).await;

            return;
        }

        self.remaining -= 1;
    }
}

impl Default for Ratelimit {
    fn default() -> Self {
        Self::new()
    }
}
