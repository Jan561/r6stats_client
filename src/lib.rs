#[macro_use]
mod macros;

pub mod http;
pub mod leaderboard;
pub mod stats;

mod client;
mod error;
mod platform;
mod region;

pub(crate) mod pointer;

pub use crate::client::Client;
pub use crate::error::Error;
pub use crate::platform::Platform;
pub use crate::region::Region;

pub(crate) use crate::http::Http;
pub(crate) use crate::pointer::Pointer;

use crate::leaderboard::Client as LeaderboardClient;
use crate::stats::Client as StatsClient;
