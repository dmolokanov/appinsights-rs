use std::env;
use std::time::Duration;

use appinsights::blocking::TelemetryClient;
use log::LevelFilter;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    let i_key = env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");

    let ai = TelemetryClient::new(i_key);

    for x in 1..=25 {
        ai.track_event(format!("Client connected: {}", x));
        std::thread::sleep(Duration::from_millis(300));

        if x == 2 {
            ai.flush_channel();
        }
    }

    ai.close_channel();
}
