mod channel;
mod client;
mod config;
mod context;
mod contracts;
pub mod telemetry;

pub use client::TelemetryClient;
pub use config::Config;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
