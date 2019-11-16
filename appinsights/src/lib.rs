mod channel;
mod client;
mod config;
mod context;
mod contracts;
pub mod telemetry;
mod time;
mod uuid;

pub use client::TelemetryClient;
pub use config::Config;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
