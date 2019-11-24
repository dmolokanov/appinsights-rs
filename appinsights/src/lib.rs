//! # Application Insights for Rust
//! The Application Insights for Rust provides an SDK to instrument your application with telemetry
//! and monitor it using Azure Portal features.
//!
//! The following Application Insights telemetry items are supported:
//! * [Availability telemetry](telemetry/struct.AvailabilityTelemetry.html)
//! * [Event telemetry](telemetry/struct.EventTelemetry.html)
//! * [Page view telemetry](telemetry/struct.PageViewTelemetry.html)
//! * [Remote dependency telemetry](telemetry/struct.RemoteDependencyTelemetry.html)
//! * [Request telemetry](telemetry/struct.RequestTelemetry.html)
//! * [Trace telemetry](telemetry/struct.TraceTelemetry.html)
//!
//! Eventually all telemetry items that Application Insights supports will be implemented.
//!
//! ## Requirements
//! Add appinsights crate to your project
//!
//! ```bash
//! $ cargo add appinsights
//! ```
//!
//! Obtain Instrumentation Key by creating a new instance of [Application Insights](https://docs.microsoft.com/en-us/azure/azure-monitor/app/create-new-resource)
//! service.
//!
//! ## Examples
//!
//! 1. Create an new instance of [TelemetryClient](struct.TelemetryClient.html) with an
//! Instrumentation Key and default settings. To get more control over client behavior please visit
//! [TelemetryConfig](struct.TelemetryConfig.html).
//! 2. Send an event telemetry to the Application Inisights service.
//!
//! ```rust
//! use appinsights::TelemetryClient;
//!
//! fn main() {
//!     // configure telemetry client with default settings
//!     let client = TelemetryClient::new("<instrumentation key>".to_string());
//!
//!     // send event telemetry to the Application Insights server
//!     client.track_event("Application started".to_string());
//! }
//! ```
//!
//! If you need more control over the client's behavior, you can create a new instance of [TelemetryConfig](struct.TelemetryConfig.html)
//! and initilize a [TelemetryClient](struct.TelemetryClient.html) with it.
//! ```rust
//! use std::time::Duration;
//! use appinsights::{TelemetryClient, TelemetryConfig};
//! use appinsights::telemetry::SeverityLevel;
//!
//! fn main() {
//!     // configure telemetry config with custom settings
//!     let config = TelemetryConfig::builder()
//!         // provide an instrumentation key for a client
//!         .i_key("<instrumentation key>")
//!         // set a new maximum time to wait until data will be sent to the server
//!         .interval(Duration::from_secs(5))
//!         // construct a new instance of telemetry configuration
//!         .build();
//!
//!     // configure telemetry client with default settings
//!     let client = TelemetryClient::from_config(config);
//!
//!     // send trace telemetry to the Application Insights server
//!     client.track_trace("A database error occurred".to_string(), SeverityLevel::Warning);
//! }
//! ```

mod channel;
mod client;
pub mod config;
mod context;
mod contracts;
pub mod telemetry;
mod time;
mod transmitter;
mod uuid;

pub use channel::TelemetryChannel;
pub use client::TelemetryClient;
pub use config::TelemetryConfig;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
