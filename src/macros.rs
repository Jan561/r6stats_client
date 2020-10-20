macro_rules! api {
    ($e:expr) => {
        concat!("https://api2.r6stats.com/public-api", $e)
    }
}
