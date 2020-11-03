use r6stats_client::{Client, Platform};
use std::env;

#[tokio::test]
async fn test_stats() {
    let token = env::var("R6STATS_TOKEN").expect("API key not found in env.");
    let client = Client::new(&token).expect("Error creating client.");

    let _ = client
        .stats()
        .generic("pengu.g2", Platform::Pc)
        .await
        .unwrap();
    let _ = client
        .stats()
        .seasonal("i jefe l", Platform::Xbox)
        .await
        .unwrap();
    let _ = client
        .stats()
        .operators("WhyGunner", Platform::Playstation)
        .await
        .unwrap();
    let _ = client
        .stats()
        .weapon_categories("pengu.g2", Platform::Pc)
        .await
        .unwrap();
    let _ = client
        .stats()
        .weapons("pengu.g2", Platform::Pc)
        .await
        .unwrap();
}
