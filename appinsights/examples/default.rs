use std::env;

use appinsights::TelemetryClient;

fn main() {
    let instrumentation_key =
        env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");

    let client = TelemetryClient::new(instrumentation_key);

    client.track_event("Client connected");
}
