use r6stats_client::{Client, Platform};
use std::env;

#[tokio::test]
async fn test_leaderboard() {
    let token = env::var("R6STATS_TOKEN").expect("API key not in env.");
    let client = Client::new(&token).expect("Error creating client.");

    let _ = client.leaderboard().get(Platform::Pc, None).await.unwrap();
    let _ = client
        .leaderboard()
        .get(Platform::Xbox, None)
        .await
        .unwrap();
    let _ = client
        .leaderboard()
        .get(Platform::Playstation, None)
        .await
        .unwrap();
}
