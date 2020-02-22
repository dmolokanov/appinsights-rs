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
//! 1. Create an new instance of [`TelemetryClient`](struct.TelemetryClient.html) with an
//! Instrumentation Key and default settings. To get more control over client behavior please visit
//! [`TelemetryConfig`](struct.TelemetryConfig.html).
//! 2. Send an event telemetry to the Application Insights service.
//!
//! ```rust
//! use appinsights::TelemetryClient;
//!
//! // configure telemetry client with default settings
//! let client = TelemetryClient::new("<instrumentation key>".to_string());
//!
//! // send event telemetry to the Application Insights server
//! client.track_event("Application started");
//! ```
//!
//! If you need more control over the client's behavior, you can create a new instance of
//! [`TelemetryConfig`](struct.TelemetryConfig.html) and initialize a
//! [`TelemetryClient`](struct.TelemetryClient.html) with it.
//!
//! ```rust
//! use std::time::Duration;
//! use appinsights::{TelemetryClient, TelemetryConfig};
//! use appinsights::telemetry::SeverityLevel;
//!
//! // configure telemetry config with custom settings
//! let config = TelemetryConfig::builder()
//! // provide an instrumentation key for a client
//!     .i_key("<instrumentation key>")
//!     // set a new maximum time to wait until data will be sent to the server
//!     .interval(Duration::from_secs(5))
//!     // construct a new instance of telemetry configuration
//!     .build();
//!
//! // configure telemetry client with default settings
//! let client = TelemetryClient::from_config(config);
//!
//! // send trace telemetry to the Application Insights server
//! client.track_trace("A database error occurred", SeverityLevel::Warning);
//! ```
//!
//! ## Telemetry submission
//!
//! A [`TelemetryClient`](struct.TelemetryClient.html) has several convenient methods to submit telemetry items.
//! * [track_event](struct.TelemetryClient.html#method.track_event) to log user action with the event name.
//! * [track_trace](struct.TelemetryClient.html#method.track_trace) to log a trace message with severity level.
//! * [track_metric](struct.TelemetryClient.html#method.track_metric) to log a numeric value that is not specified with a specific event.
//! * [track_request](struct.TelemetryClient.html#method.track_request) to log a HTTP request with the specified method, URL, duration and response code.
//! * [track_remote_dependency](struct.TelemetryClient.html#method.track_remote_dependency) to log a dependency with the specified name, type, target, and success status.
//! * [track_availability](struct.TelemetryClient.html#method.track_availability) to log an availability test result with the specified test name, duration, and success status.
//!
//! But they provide the very basic set of parameters telemetry types can represent. For example all
//! telemetry items support [`properties`](telemetry/trait.Telemetry.html#method.properties) and
//! [`tags`](telemetry/trait.Telemetry.html#method.tags) which not accessible via these methods.
//! More complete versions are available through use of _telemetry item_ struct which can be
//! submitted through the [`track`](struct.TelemetryClient.html#method.track) method.
//!
//! ## Context tags
//!
//! All telemetry items have [`tags`](telemetry/trait.Telemetry.html#method.tags) field that can
//! contain an information _about_ telemetry such as user, session, and device information.
//! The type of `tags` is [`ContextTags`](telemetry/struct.ContextTags.html) that contains
//! convenient methods to store and and retrieve commonly used data.
//! A [`TelemetryContext`](struct.TelemetryContext.html) also have
//! [`tags`](struct.TelemetryContext.html#method.tags). These tags will be applied to all telemetry
//! items submitted to a server. In case when some tags exists in both telemetry client context tags
//! and telemetry item tags, later will be sent to the server.
//!
//! ```rust
//! use std::time::Duration;
//! use http::Method;
//! use appinsights::TelemetryClient;
//! use appinsights::telemetry::{RequestTelemetry, Telemetry};
//!
//! // configure telemetry with default settings
//! let mut client = TelemetryClient::new("instrumentation".to_string());
//!
//! // set role instance name globally. This is usually the name of the service submitting the telemetry
//! client.context_mut().tags_mut().cloud_mut().set_role("rust_server".to_string());
//!
//! // set the role instance to the host name. Note that this is done automatically by the SDK
//! client.context_mut().tags_mut().cloud_mut().set_role_instance("rust_server".to_string());
//!
//! let mut telemetry = RequestTelemetry::new(
//!     Method::GET,
//!     "https://example.com/main.html".parse().unwrap(),
//!     Duration::from_secs(2),
//!     "200",
//! );
//!
//! // set the account id context tag
//! telemetry.tags_mut().user_mut().set_account_id("account_id".to_string());
//!
//! // send telemetry to the Application Insights server
//! client.track(telemetry);
//! ```
//!
//! ## Common properties
//!
//! All telemetry items have [`properties`](telemetry/trait.Telemetry.html#method.properties) field
//! and a [`TelemetryContext`](struct.TelemetryContext.html) also have [`properties`](struct.TelemetryContext.html#method.properties).
//! These properties will be applied to all telemetry items submitted to a server. In case when some properties exists in both telemetry
//! client properties and telemetry item properties, later will be sent to the server.
//!
//! ```rust
//! use appinsights::TelemetryClient;
//! use appinsights::telemetry::{RequestTelemetry, Telemetry};
//!
//! // configure telemetry with default settings
//! let mut client = TelemetryClient::new("instrumentation".to_string());
//!
//! // set custom telemetry item property globally
//! client.context_mut().properties_mut().insert("Resource Group".to_string(), "my-rg".to_string());
//!
//! // ... send telemetry to the Application Insights server
//! ```
//!
//! ## Shutdown
//!
//! The telemetry item submission happens asynchronously. The internal channel starts a new worker
//! thread that used to accept and send telemetry to the server. While telemetry is not sent the
//! worker stores it in memory, so when application crashes the data will be lost. Luckily SDK
//! provides several convenient methods to deal with this issue.
//! * [`flush_channel`](struct.TelemetryClient.html#method.flush_channel) will trigger telemetry submission
//! as soon as possible. It returns immediately and telemetry is no guaranteed to be sent.
//! * [`close_channel`](struct.TelemetryClient.html#method.close_channel) will cause the channel to
//! stop accepting any new telemetry items, submit all pending ones, block current thread and
//! wait until data will be sent at most once. If telemetry submission fails, it will not retry.
//! This method consumes the value of client so it makes impossible to use a client with close channel.
//! * Once [`TelemetryClient`](struct.TelemetryClient.html) is out of scope `drop` method for channel
//! will be called. It will trigger termination of submission flow, all pending items discarded,
//! block current thread until all resources freed. It is __default__ "exit" mode for client.
#![deny(unused_extern_crates)]
#![deny(missing_docs)]

mod channel;
mod client;
mod config;
mod context;
mod contracts;
pub mod telemetry;
mod time;
mod timeout;
mod transmitter;
mod uuid;

pub use channel::InMemoryChannel;
pub use client::TelemetryClient;
#[doc(inline)]
pub use config::TelemetryConfig;
pub use context::TelemetryContext;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
