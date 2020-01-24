# Application Insights for Rust
[![build-status](https://github.com/dmolokanov/appinsights-rs/workflows/CI/badge.svg)](https://github.com/dmolokanov/appinsights-rs/actions)
[![crate-status](https://img.shields.io/crates/v/appinsights.svg)](https://crates.io/crates/appinsights)
[![docs-status](https://docs.rs/appinsights/badge.svg)](https://docs.rs/appinsights)
[![dependabot-status](https://api.dependabot.com/badges/status?host=github&repo=dmolokanov/appinsights-rs)](https://dependabot.com)

This project provides a Rust SDK for [Application Insights](http://azure.microsoft.com/en-us/services/application-insights/). Application Insights is an APM service that helps to monitor running applications. This Rust crate allows to send various kinds of telemetry information to the server to be visualized later on Azure Portal. 

> :triangular_flag_on_post: **Disclaimer**  
> This project is not an officially recognized Microsoft product and is not an endorsement of any future product offering from Microsoft.
>
> _Microsoft and Azure are registered trademarks of Microsoft Corporation._

## Installation
```bash
$ cargo add appinsights
```
or just add this to your `Cargo.toml`:

```toml
[dependencies]
appinisghts = "0.1"
```

## Usage

To start tracking telemetry for your application first thing you need to do is to obtain an [Instrumentation Key](https://docs.microsoft.com/en-us/azure/azure-monitor/app/create-new-resource) and initialize `TelemetryClient` with it.

This client will be used to send all telemetry data to Application Insights. This SDK doesn't collect any telemetry automatically, so this client should be used everywhere in the code to report health information about an application. 

```rust
use appinsights::TelemetryClient;

fn main() {
    // configure telemetry client with default settings
    let client = TelemetryClient::new("<instrumentation key>");
    
    // send event telemetry to the Application Insights server
    client.track_event("application started");
}
```
If you need more control over the client's behavior, you can create a new instance of `TelemetryConfig` and initialize a `TelemetryClient` with it.

```rust
use std::time::Duration;
use appinsights::{TelemetryClient, TelemetryConfig};
use appinsights::telemetry::SeverityLevel;

fn main() {
    // configure telemetry config with custom settings
    let config = TelemetryConfig::builder()
        // provide an instrumentation key for a client
        .i_key("<instrumentation key>")
        // set a new maximum time to wait until data will be sent to the server
        .interval(Duration::from_secs(5))
        // construct a new instance of telemetry configuration
        .build();

    // configure telemetry client with default settings
    let client = TelemetryClient::from_config(config);

    // send trace telemetry to the Application Insights server
    client.track_trace("A database error occurred".to_string(), SeverityLevel::Warning);
}
```

## License
This project is licensed under the terms of the [MIT](LICENSE) license.