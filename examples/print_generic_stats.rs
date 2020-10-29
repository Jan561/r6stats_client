use r6stats_client::{Client, Platform};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("R6STATS_TOKEN").expect("API Token not found in env");
    let client = Client::new(&token).expect("Error creating client");

    let stats = client
        .stats()
        .generic("pengu.g2", Platform::Pc)
        .await
        .expect("Error getting stats");

    println!("{:#?}", stats);
}
