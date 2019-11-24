# Application Insights for Rust
[![build](https://github.com/dmolokanov/appinsights-rs/workflows/CI/badge.svg)](https://github.com/dmolokanov/appinsights-rs/actions)

This project provides a Rust SDK for [Application Insights](http://azure.microsoft.com/en-us/services/application-insights/). Application Insights is an APM service that helps to monitor running applications. This Rust crate allows to send various kinds of telemetry information to the server to be visualized later on Azure Portal. 

> :triangular_flag_on_post: **Disclaimer**  
> This project is not an officially recognized Microsoft product and is not an endorsement of any future product offering from Microsoft.
>
> _Microsoft and Azure are registered trademarks of Microsoft Corporation._

## Installation
```sh
$ cargo add appinsights
```
or just add this to your `Cargo.toml`:

```toml
[dependencies]
appinisghts = "0.1"
```

## Usage

To start tracking telemetry for your application first thing you need to do is to obtain an [Instrumentation Key](https://docs.microsoft.com/en-us/azure/azure-monitor/app/create-new-resource) and initialize TelemetryClient with it.

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

