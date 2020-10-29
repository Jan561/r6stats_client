use r6stats_client::stats::model::seasonal::Season;
use r6stats_client::{Client, Platform, Region};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("R6STATS_TOKEN").expect("API Token not found in env");
    let client = Client::new(&token).expect("Error building client");

    let seasonal_stats = client
        .stats()
        .seasonal("pengu.g2", Platform::Pc)
        .await
        .expect("Error getting stats.");

    let rank = seasonal_stats
        .seasons
        .get(&Season::ShadowLegacy)
        .and_then(|s| s.regions.get(&Region::Emea))
        .and_then(|r| r.get(0))
        .map(|r| r.rank_text.clone())
        .expect("Couldn't get rank of current season.");

    println!("{}", rank);
}
