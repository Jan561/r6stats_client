# [r6stats] Client

This crate provides a client for the r6stats API. It supports the `/stats` and the `/leaderboard` endpoint.

## Usage
Add the following to your `Cargo.toml`:

```toml
[dependencies]
r6stats_client = "0.1"
```
Basic example:
```rust
#[tokio::main]
async fn main() {
    // You need an API key to access the endpoints of r6stats.
    //
    // If you don't have one, you can request one from their support.
    let token = "<API KEY HERE>";
    let client = Client::new(token).unwrap();

    let leaderboard = client
        .leaderboard()
        .get(Platform::Pc, Some(Region::Emea))
        .await
        .unwrap();

    println!("{:#?}", leaderboard);
}
```

[R6Stats]: https://r6stats.com
