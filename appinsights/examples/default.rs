use std::env;

use appinsights::TelemetryClient;

fn main() {
    let i_key = env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");

    let client = TelemetryClient::new(i_key);

    client.track_event("Client connected");

    std::thread::sleep(std::time::Duration::from_secs(2));
}
