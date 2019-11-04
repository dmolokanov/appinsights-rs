use std::env;

use appinsights::TelemetryClient;

fn main() {
    let ikey = env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");

    let client = TelemetryClient::new(ikey);

    client.track_event("Client connected");

    std::thread::sleep(std::time::Duration::from_secs(2));
}
