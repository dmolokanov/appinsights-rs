use std::env;
use std::error::Error;

use appinsights::{TelemetryChannel, TelemetryClient};
use log::LevelFilter;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder().filter_level(LevelFilter::Trace).init();

    let i_key = env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");

    let client = TelemetryClient::new(i_key);

    for x in 1..25 {
        client.track_event(format!("Client connected: {}", x))?;
        std::thread::sleep(Duration::from_millis(300));

        if x == 2 {
            client.channel().flush()?;
        }
    }

    client.close_channel()?;
    Ok(())
}
