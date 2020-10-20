use r6stats_client::{Client, Platform};
use std::env;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let token = env::var("R6STATS_TOKEN").expect("API Token not found in env.");

    // Create client with ratelimit of 120 requests per 30 seconds
    let client = Client::with_ratelimit(&token, |r| r.limit(120).interval(Duration::from_secs(30)))
        .expect("Error creating client.");

    println!("Executing 120 requests.");

    // No (pre-)ratelimiting
    for _ in 0..120 {
        println!("{:?}", client.ratelimit().await);
        let _ = client.leaderboard().get(Platform::Pc, None).await;
    }

    println!("Executing 121st request.");
    println!("{:?}", client.ratelimit().await);

    // Preratelimited. Request on hold until next ratelimit interval
    let _ = client.leaderboard().get(Platform::Pc, None).await;

    println!("Finished 121st request.");
}
