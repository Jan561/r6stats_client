//! The stats endpoint of the api.

pub mod model;

mod client;
mod http;
mod kind;

pub use self::client::Client;

use self::kind::Kind;
