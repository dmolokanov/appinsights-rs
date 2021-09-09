mod logger;

use std::{
    env,
    sync::{Arc, RwLock},
    time::Duration,
};

use appinsights::{blocking::TelemetryClient, telemetry::SeverityLevel};
use hyper::{Method, Uri};

#[test]
fn it_tracks_all_telemetry_items() {
    let entries = Arc::new(RwLock::new(Vec::new()));
    logger::builder(entries.clone()).output(true).init();

    let i_key = env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");
    let ai = TelemetryClient::new(i_key);

    ai.track_event("event happened");
    ai.track_trace("Unable to connect to a gateway", SeverityLevel::Warning);
    ai.track_metric("gateway_latency_ms", 113.0);
    ai.track_request(
        Method::GET,
        "https://api.github.com/dmolokanov/appinsights-rs"
            .parse::<Uri>()
            .unwrap(),
        Duration::from_millis(100),
        "200".to_string(),
    );
    ai.track_remote_dependency(
        "GET https://api.github.com/dmolokanov/appinsights-rs",
        "HTTP",
        "api.github.com",
        true,
    );
    ai.track_availability(
        "GET https://api.github.com/dmolokanov/appinsights-rs",
        Duration::from_secs(2),
        true,
    );

    ai.close_channel();

    logger::wait_until_blocking(&entries, "Successfully sent 6 items", Duration::from_secs(10));
}
