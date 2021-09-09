//! A blocking TelemetryClient API.
//!
//! The blocking `TelemetryClient` will block the current thread to execute, instead
//! of returning futures that need to be executed on a runtime.
//!
//! Conversely, the functionality in `appinsights::blocking` must *not* be executed
//! within an runtime, or it will panic when attempting to block. If
//! calling directly from an function, consider using an async
//! [`appinsights::TelemetryClient`][crate::TelemetryClient] instead.
//!
//! ```rust
//! use appinsights::blocking::TelemetryClient;
//!
//! // configure telemetry client with default settings
//! let client = TelemetryClient::new("<instrumentation key>".to_string());
//!
//! // send event telemetry to the Application Insights server
//! client.track_event("Application started");
//!
//! // stop the client
//! // NOT it will **block** the current thread until
//! client.close_channel();
//! ```

use std::{fmt::Display, time::Duration};

use http::{Method, Uri};
use log::debug;
use tokio::sync::mpsc;

use crate::{
    channel::{InMemoryChannel, TelemetryChannel},
    contracts::Envelope,
    telemetry::{
        AvailabilityTelemetry, EventTelemetry, MetricTelemetry, RemoteDependencyTelemetry, RequestTelemetry,
        SeverityLevel, Telemetry, TraceTelemetry,
    },
    TelemetryConfig, TelemetryContext,
};

/// A blocking version of Application Insights telemetry client. It provides an interface to track telemetry items.
pub struct TelemetryClient {
    inner: ChannelHandle,
}

impl TelemetryClient {
    /// Creates a new telemetry client that submits telemetry with specified instrumentation key.
    pub fn new(i_key: String) -> Self {
        Self::from_config(TelemetryConfig::new(i_key))
    }

    /// Creates a new telemetry client configured with specified configuration.
    pub fn from_config(config: TelemetryConfig) -> Self {
        Self::create(config, |config| InMemoryChannel::new(config))
    }

    pub(crate) fn create<C, F>(config: TelemetryConfig, channel: F) -> Self
    where
        C: TelemetryChannel,
        F: FnOnce(&TelemetryConfig) -> C + Send + 'static,
    {
        let inner = ChannelHandle::new(config, channel);
        Self { inner }
    }

    /// Determines whether this client is enabled and will accept telemetry.
    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    /// Enables or disables telemetry client. When disabled, telemetry is silently swallowed by the client. Defaults to enabled.
    pub fn enabled(&mut self, enabled: bool) {
        self.inner.enabled(enabled);
    }

    /// Returns an immutable reference to a collection of tag data to attach to the telemetry item.
    pub fn context(&self) -> &TelemetryContext {
        &self.inner.context
    }

    /// Returns a mutable reference to a collection of tag data to attach to the telemetry item.
    pub fn context_mut(&mut self) -> &mut TelemetryContext {
        &mut self.inner.context
    }

    /// Logs a user action with the specified name.
    pub fn track_event(&self, name: impl Into<String>) {
        let event = EventTelemetry::new(name);
        self.track(event)
    }

    /// Logs a trace message with a specified severity level.
    pub fn track_trace(&self, message: impl Into<String>, severity: SeverityLevel) {
        let event = TraceTelemetry::new(message, severity);
        self.track(event)
    }

    /// Logs a numeric value that is not specified with a specific event.
    /// Typically used to send regular reports of performance indicators.
    pub fn track_metric(&self, name: impl Into<String>, value: f64) {
        let event = MetricTelemetry::new(name, value);
        self.track(event)
    }

    /// Logs a HTTP request with the specified method, URL, duration and response code.
    pub fn track_request(&self, method: Method, uri: Uri, duration: Duration, response_code: impl Into<String>) {
        let event = RequestTelemetry::new(method, uri, duration, response_code);
        self.track(event)
    }

    /// Logs a dependency with the specified name, type, target, and success status.
    pub fn track_remote_dependency(
        &self,
        name: impl Into<String>,
        dependency_type: impl Into<String>,
        target: impl Into<String>,
        success: bool,
    ) {
        let event = RemoteDependencyTelemetry::new(name, dependency_type, Default::default(), target, success);
        self.track(event)
    }

    /// Logs an availability test result with the specified test name, duration, and success status.
    pub fn track_availability(&self, name: impl Into<String>, duration: Duration, success: bool) {
        let event = AvailabilityTelemetry::new(name, duration, success);
        self.track(event)
    }

    /// Submits a specific telemetry event.
    pub fn track<E>(&self, event: E)
    where
        E: Telemetry,
        (TelemetryContext, E): Into<Envelope>,
    {
        self.inner.track(event);
    }

    /// Forces all pending telemetry items to be submitted. The current thread will not be blocked.
    pub fn flush_channel(&self) {
        self.inner.flush();
    }

    /// Flushes and tears down the submission flow and closes internal channels.
    /// It blocks the current thread until all pending telemetry items have been submitted and it is safe to
    /// shutdown without losing telemetry.
    /// This method consumes the value of client so it makes impossible to use a client with close
    /// channel.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// // send heartbeats while application is running
    /// let running = true;
    /// while running {
    ///     client.track_event("app is running");
    /// }
    ///
    /// // wait until pending telemetry is sent at most once and tear down submission flow
    /// client.close_channel();
    ///
    /// // unable to sent any telemetry after client closes its channel
    /// // client.track_event("app is stopped".to_string());
    /// ```
    pub fn close_channel(self) {
        self.inner.close();
    }

    /// Tears down the submission flow and closes internal channels.
    /// Any telemetry waiting to be sent is discarded. This is a more abrupt version of [`close_channel`](#method.close_channel).
    /// This method consumes the value of client so it makes impossible to use a client with close
    /// channel.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// // send heartbeats while application is running
    /// let running = true;
    /// while running {
    ///     client.track_event("app is running");
    /// }
    ///
    /// // wait until pending telemetry is sent at most once and tear down submission flow
    /// // or just drop(client)
    /// client.terminate();
    ///
    /// // unable to sent any telemetry after client closes its channel
    /// // client.track_event("app is stopped".to_string());
    /// ```
    pub fn terminate(self) {}
}

struct ChannelHandle {
    enabled: bool,
    context: TelemetryContext,
    inner: InnerChannelHandle,
}

impl ChannelHandle {
    fn new<C, F>(config: TelemetryConfig, channel: F) -> Self
    where
        C: TelemetryChannel,
        F: FnOnce(&TelemetryConfig) -> C + Send + 'static,
    {
        let context = TelemetryContext::from_config(&config);

        let (tx, mut rx) = mpsc::unbounded_channel::<(ClientCommand, OneshotResponse)>();

        let handle = std::thread::Builder::new()
            .name("appinsights-internal-sync-runtime".into())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("tokio runtime");

                let f = async move {
                    let mut channel = channel(&config);

                    while let Some((command, req_tx)) = rx.recv().await {
                        match command {
                            ClientCommand::Envelope(envelop) => channel.send(envelop),
                            ClientCommand::Flush => channel.flush(),
                            ClientCommand::Stop => channel.close().await,
                            ClientCommand::Terminate => channel.terminate().await,
                        }
                        let _ = req_tx.send(());
                    }
                };
                rt.block_on(f);
            })
            .expect("failed to create a thread");

        let inner = InnerChannelHandle {
            tx: Some(tx),
            thread: Some(handle),
        };

        ChannelHandle {
            inner,
            enabled: true,
            context,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn track<E>(&self, event: E)
    where
        E: Telemetry,
        (TelemetryContext, E): Into<Envelope>,
    {
        if self.is_enabled() {
            let envelop = (self.context.clone(), event).into();
            let command = ClientCommand::Envelope(envelop);

            let (tx, mut rx) = mpsc::channel(1);

            self.inner
                .tx
                .as_ref()
                .expect("sync thread exited early")
                .send((command, tx))
                .expect("sync thread panicked");

            let _ = rx.blocking_recv();
        }
    }

    fn flush(&self) {
        self.inner.flush();
    }

    fn close(mut self) {
        self.inner.shutdown(ClientCommand::Stop)
    }
}

type OneshotResponse = mpsc::Sender<()>;

type ThreadSender = mpsc::UnboundedSender<(ClientCommand, OneshotResponse)>;

struct InnerChannelHandle {
    tx: Option<ThreadSender>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl InnerChannelHandle {
    fn flush(&self) {
        if let Some(sender) = &self.tx {
            send_command(sender, ClientCommand::Flush);
        }
    }

    fn shutdown(&mut self, command: ClientCommand) {
        if let Some(sender) = self.tx.take() {
            send_command(&sender, command);
        }

        self.thread.take().map(|h| h.join());
    }
}

impl Drop for InnerChannelHandle {
    fn drop(&mut self) {
        self.shutdown(ClientCommand::Terminate)
    }
}

fn send_command(sender: &ThreadSender, command: ClientCommand) {
    debug!("Sending {} command to channel", command);
    let (tx, mut rx) = mpsc::channel(1);
    sender.send((command, tx)).expect("sync thread panicked?");

    let _ = rx.blocking_recv();
}

#[derive(Debug, Clone)]
enum ClientCommand {
    Envelope(Envelope),
    Flush,
    Stop,
    Terminate,
}

impl Display for ClientCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ClientCommand::Envelope(_) => "event",
            ClientCommand::Flush => "flush",
            ClientCommand::Stop => "stop",
            ClientCommand::Terminate => "terminate",
        };

        write!(f, "{}", message)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crossbeam_queue::SegQueue;
    use matches::assert_matches;

    use super::*;
    use crate::client::tests::{TestChannel, TestTelemetry};

    #[test]
    fn it_enabled_by_default() {
        let client = TelemetryClient::new("key".into());
        assert!(client.is_enabled())
    }

    #[test]
    fn it_disables_telemetry() {
        let mut client = TelemetryClient::new("key".into());

        client.enabled(false);

        assert!(!client.is_enabled())
    }

    #[test]
    fn it_submits_telemetry() {
        let events = Arc::new(SegQueue::default());
        let client = create_client(events.clone());

        client.track(TestTelemetry {});

        assert_eq!(events.len(), 1)
    }

    #[test]
    fn it_swallows_telemetry_when_disabled() {
        let events = Arc::new(SegQueue::default());
        let mut client = create_client(events.clone());
        client.enabled(false);

        client.track(TestTelemetry {});

        assert!(events.is_empty())
    }

    #[test]
    fn it_creates_client_with_default_tags() {
        let client = TelemetryClient::new("instrumentation".into());

        let tags = client.context().tags();
        assert_matches!(tags.internal().sdk_version(), Some(version) if version.starts_with("rust"));
        assert_matches!(tags.device().os_version(), Some(_))
    }

    #[test]
    fn it_does_not_fail_with_tokio() {
        let client = TelemetryClient::new("instrumentation".into());
        assert!(client.is_enabled())
    }

    fn create_client(events: Arc<SegQueue<Envelope>>) -> TelemetryClient {
        let config = TelemetryConfig::new("instrumentation".into());
        TelemetryClient::create(config, |_| TestChannel::new(events))
    }
}

#[cfg(test)]
mod integration_tests;
