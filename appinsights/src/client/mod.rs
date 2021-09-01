use std::time::Duration;

use http::{Method, Uri};

use crate::{
    channel::{InMemoryChannel, TelemetryChannel},
    context::TelemetryContext,
    contracts::Envelope,
    telemetry::{
        AvailabilityTelemetry, EventTelemetry, MetricTelemetry, RemoteDependencyTelemetry, RequestTelemetry,
        SeverityLevel, Telemetry, TraceTelemetry,
    },
    TelemetryConfig,
};

/// Application Insights telemetry client provides an interface to track telemetry items.
pub struct TelemetryClient {
    enabled: bool,
    context: TelemetryContext,
    channel: Box<dyn TelemetryChannel>,
}

impl TelemetryClient {
    /// Creates a new telemetry client that submits telemetry with specified instrumentation key.
    pub fn new(i_key: String) -> Self {
        Self::from_config(TelemetryConfig::new(i_key))
    }

    /// Creates a new telemetry client configured with specified configuration.
    pub fn from_config(config: TelemetryConfig) -> Self {
        Self::create(&config, InMemoryChannel::new(&config))
    }

    /// Creates a new telemetry client with custom telemetry channel.
    pub(crate) fn create<C: TelemetryChannel + 'static>(config: &TelemetryConfig, channel: C) -> Self {
        Self {
            enabled: true,
            context: TelemetryContext::from_config(config),
            channel: Box::new(channel),
        }
    }

    /// Determines whether this client is enabled and will accept telemetry.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// assert!(client.is_enabled());
    /// ```
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables or disables telemetry client. When disabled, telemetry is silently swallowed by the client. Defaults to enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use appinsights::TelemetryClient;
    /// let mut client = TelemetryClient::new("<instrumentation key>".to_string());
    /// assert!(client.is_enabled());
    ///
    /// client.enabled(false);
    /// assert_eq!(client.is_enabled(), false);
    /// ```
    pub fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns an immutable reference to a collection of tag data to attach to the telemetry item.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use appinsights::TelemetryClient;
    /// let mut client = TelemetryClient::new("<instrumentation key>".to_string());
    /// client.context_mut().tags_mut().cloud_mut().set_role("rust_server".to_string());
    ///
    /// assert_eq!(client.context().tags().cloud().role(), Some("rust_server"));
    /// ```
    pub fn context(&self) -> &TelemetryContext {
        &self.context
    }

    /// Returns a mutable reference to a collection of tag data to attach to the telemetry item.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use appinsights::TelemetryClient;
    /// let mut client = TelemetryClient::new("<instrumentation key>".to_string());
    /// client.context_mut().tags_mut().insert("app_version".into(), "v0.1.1".to_string());
    /// client.context_mut().properties_mut().insert("Resource Group".into(), "my-rg".to_string());
    ///
    /// assert_eq!(client.context().tags().get("app_version"), Some(&"v0.1.1".to_string()));
    /// assert_eq!(client.context().properties().get("Resource Group"), Some(&"my-rg".to_string()));
    /// ```
    pub fn context_mut(&mut self) -> &mut TelemetryContext {
        &mut self.context
    }

    /// Logs a user action with the specified name.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// client.track_event("app is running");
    /// ```
    pub fn track_event(&self, name: impl Into<String>) {
        let event = EventTelemetry::new(name);
        self.track(event)
    }

    /// Logs a trace message with a specified severity level.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # use appinsights::telemetry::SeverityLevel;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// client.track_trace("Unable to connect to a gateway", SeverityLevel::Warning);
    /// ```
    pub fn track_trace(&self, message: impl Into<String>, severity: SeverityLevel) {
        let event = TraceTelemetry::new(message, severity);
        self.track(event)
    }

    /// Logs a numeric value that is not specified with a specific event.
    /// Typically used to send regular reports of performance indicators.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # use appinsights::telemetry::SeverityLevel;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// client.track_metric("gateway_latency_ms", 113.0);
    /// ```    
    pub fn track_metric(&self, name: impl Into<String>, value: f64) {
        let event = MetricTelemetry::new(name, value);
        self.track(event)
    }

    /// Logs a HTTP request with the specified method, URL, duration and response code.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// use http::{Method, Uri};
    /// use std::time::Duration;
    ///
    /// let uri: Uri = "https://api.github.com/dmolokanov/appinsights-rs".parse().unwrap();
    /// client.track_request(Method::GET, uri, Duration::from_millis(100), "200");
    /// ```
    pub fn track_request(&self, method: Method, uri: Uri, duration: Duration, response_code: impl Into<String>) {
        let event = RequestTelemetry::new(method, uri, duration, response_code);
        self.track(event)
    }

    /// Logs a dependency with the specified name, type, target, and success status.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// client.track_remote_dependency(
    ///     "GET https://api.github.com/dmolokanov/appinsights-rs",
    ///     "HTTP",
    ///     "api.github.com",
    ///     true
    /// );
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// use std::time::Duration;
    ///
    /// client.track_availability(
    ///     "GET https://api.github.com/dmolokanov/appinsights-rs",
    ///     Duration::from_millis(100),
    ///     true
    /// );
    /// ```
    pub fn track_availability(&self, name: impl Into<String>, duration: Duration, success: bool) {
        let event = AvailabilityTelemetry::new(name, duration, success);
        self.track(event)
    }

    /// Submits a specific telemetry event.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// use appinsights::telemetry::AggregateMetricTelemetry;
    ///
    /// let mut telemetry = AggregateMetricTelemetry::new("device_message_latency_per_min");
    /// telemetry.stats_mut().add_data(&[113.0, 250.0, 316.0]);
    ///
    /// client.track(telemetry);
    /// ```
    pub fn track<E>(&self, event: E)
    where
        E: Telemetry,
        (TelemetryContext, E): Into<Envelope>,
    {
        if self.is_enabled() {
            let envelop = (self.context.clone(), event).into();
            self.channel.send(envelop);
        }
    }

    /// Forces all pending telemetry items to be submitted. The current task will not be blocked.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// let mut counter = 0;
    ///
    /// // send heartbeats while application is running
    /// let running = true;
    /// while running {
    ///     client.track_event("app is running");
    ///     counter += 1;
    ///
    ///     // if the rate is bigger than submission interval you can make sure that data is
    ///     // triggered for submission (each 100 items)
    ///     if counter == 100 {
    ///         // trigger submission of all pending items
    ///         client.flush_channel();
    ///         counter = 0;
    ///     }
    /// }
    /// ```
    pub fn flush_channel(&self) {
        self.channel.flush();
    }

    /// Flushes and tears down the submission flow and closes internal channels.
    /// It blocks the current task until all pending telemetry items have been submitted and it is safe to
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
    /// client.close_channel().await;
    ///
    /// // unable to sent any telemetry after client closes its channel
    /// // client.track_event("app is stopped".to_string());
    /// ```
    pub async fn close_channel(mut self) {
        self.channel.close().await;
    }

    /// Tears down the submission flow and closes internal channels.
    /// Any telemetry waiting to be sent is discarded. This is a more abrupt version of [`close_channel`](#method.close_channel).
    /// This method consumes the value of client so it makes impossible to use a client with close
    /// channel.
    ///
    /// This method should be used in cases when the client should be stopped. It is a separate function until
    /// `async_drop` is implemented in rust.
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
    /// client.terminate().await;
    ///
    /// // unable to sent any telemetry after client closes its channel
    /// // client.track_event("app is stopped".to_string());
    /// ```
    pub async fn terminate(mut self) {
        self.channel.terminate().await;
    }
}

impl From<(TelemetryConfig, TelemetryContext)> for TelemetryClient {
    fn from((config, context): (TelemetryConfig, TelemetryContext)) -> Self {
        Self {
            enabled: true,
            context,
            channel: Box::new(InMemoryChannel::new(&config)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use crossbeam_queue::SegQueue;
    use matches::assert_matches;

    use super::*;
    use crate::telemetry::{ContextTags, Properties};

    #[tokio::test]
    async fn it_enabled_by_default() {
        let client = TelemetryClient::new("key".into());
        assert!(client.is_enabled())
    }

    #[tokio::test]
    async fn it_disables_telemetry() {
        let mut client = TelemetryClient::new("key".into());

        client.enabled(false);

        assert!(!client.is_enabled())
    }

    #[tokio::test]
    async fn it_submits_telemetry() {
        let events = Arc::new(SegQueue::default());
        let client = create_client(events.clone());

        client.track(TestTelemetry {});

        assert_eq!(events.len(), 1)
    }

    #[tokio::test]
    async fn it_swallows_telemetry_when_disabled() {
        let events = Arc::new(SegQueue::default());
        let mut client = create_client(events.clone());
        client.enabled(false);

        client.track(TestTelemetry {});

        assert!(events.is_empty())
    }

    #[tokio::test]
    async fn it_creates_client_with_default_tags() {
        let client = TelemetryClient::new("instrumentation".into());

        let tags = client.context().tags();
        assert_matches!(tags.internal().sdk_version(), Some(version) if version.starts_with("rust"));
        assert_matches!(tags.device().os_version(), Some(_))
    }

    #[tokio::test]
    async fn it_does_not_fail_with_tokio() {
        let client = TelemetryClient::new("instrumentation".into());
        assert!(client.is_enabled())
    }

    fn create_client(events: Arc<SegQueue<Envelope>>) -> TelemetryClient {
        let config = TelemetryConfig::new("instrumentation".into());
        TelemetryClient::create(&config, TestChannel { events })
    }

    struct TestTelemetry {}

    impl Telemetry for TestTelemetry {
        fn timestamp(&self) -> DateTime<Utc> {
            unimplemented!()
        }

        fn properties(&self) -> &Properties {
            unimplemented!()
        }

        fn properties_mut(&mut self) -> &mut Properties {
            unimplemented!()
        }

        fn tags(&self) -> &ContextTags {
            unimplemented!()
        }

        fn tags_mut(&mut self) -> &mut ContextTags {
            unimplemented!()
        }
    }

    #[derive(Clone)]
    struct TestData;

    impl From<(TelemetryContext, TestTelemetry)> for Envelope {
        fn from((_, _): (TelemetryContext, TestTelemetry)) -> Self {
            Envelope::default()
        }
    }

    struct TestChannel {
        events: Arc<SegQueue<Envelope>>,
    }

    #[async_trait]
    impl TelemetryChannel for TestChannel {
        fn send(&self, envelop: Envelope) {
            self.events.push(envelop);
        }

        fn flush(&self) {
            unimplemented!()
        }

        async fn close(&mut self) {
            unimplemented!()
        }

        async fn terminate(&mut self) {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod integration_tests;
