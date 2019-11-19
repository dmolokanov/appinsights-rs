use std::env;

use appinsights::TelemetryClient;
use log::LevelFilter;
use std::time::Duration;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    let i_key = env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");

    let client = TelemetryClient::new(i_key);

    for _ in 0..25 {
        client.track_event("Client connected".into());
        std::thread::sleep(Duration::from_millis(300));
    }
}
