//! # r6stats Client
//!
//! This crate provides a client for the r6stats API. It supports the `/stats` and the `/leaderboard` endpoint.
//!
//! ## Usage
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! r6stats_client = "0.1"
//! ```
//! Basic example:
//! ```rust
//! use r6stats_client::{Client, Platform, Region};
//!
//! #[tokio::main]
//! async fn main() {
//!     // You need an API key to access the endpoints of r6stats.
//!     //
//!     // If you don't have one, you can request one from their support.
//!     let token = "<API KEY HERE>";
//!     let client = Client::new(token);
//!
//!     let leaderboard = client.leaderboard().get(Platform::Pc, Some(Region::Emea)).await.unwrap();
//!
//!     println!("{:#?}", leaderboard);
//! }
//! ```
//!
//! ## Features
//!
//! - `ratelimiting` (default): Enables pre-ratelimiting **before** sending requests to prevent HTTP-429 Errors.
//! **Note**: Ratelimits are enforced by the server nevertheless.
//! - `threadsafe`: Makes `Client` threadsafe (`Send` + `Sync`)

#[macro_use]
pub(crate) mod internals;

pub mod http;
pub mod leaderboard;
pub mod stats;

mod client;
mod error;
mod platform;
mod region;

pub use crate::client::Client;
pub use crate::error::Error;
pub use crate::platform::Platform;
pub use crate::region::Region;

pub(crate) use crate::http::Http;
