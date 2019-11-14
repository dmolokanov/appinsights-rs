mod channel;
mod client;
mod config;
mod context;
mod contracts;
mod systemtime;
pub mod telemetry;

pub use client::TelemetryClient;
pub use config::Config;
pub use systemtime::SystemTime;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
