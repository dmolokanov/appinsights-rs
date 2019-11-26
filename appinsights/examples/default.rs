use std::env;
use std::time::Duration;

use appinsights::TelemetryClient;
use log::LevelFilter;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let i_key = env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");

    let client = TelemetryClient::new(i_key);

    for x in 1..=25 {
        client.track_event(format!("Client connected: {}", x));
        std::thread::sleep(Duration::from_millis(300));

        if x == 2 {
            client.flush_channel();
        }
    }

    client.close_channel();
}
