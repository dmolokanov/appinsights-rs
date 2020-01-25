use std::time::Duration;

use http::{Method, Uri};

use crate::channel::{InMemoryChannel, TelemetryChannel};
use crate::context::TelemetryContext;
use crate::contracts::Envelope;
use crate::telemetry::*;
use crate::TelemetryConfig;

/// Application Insights telemetry client provides an interface to track telemetry items.
pub struct TelemetryClient<C> {
    enabled: bool,
    context: TelemetryContext,
    channel: C,
}

impl TelemetryClient<InMemoryChannel> {
    /// Creates a new telemetry client that submits telemetry with specified instrumentation key.
    pub fn new(i_key: String) -> Self {
        Self::from_config(TelemetryConfig::new(i_key))
    }
}

impl TelemetryClient<InMemoryChannel> {
    /// Creates a new telemetry client configured with specified configuration.
    pub fn from_config(config: TelemetryConfig) -> Self {
        Self {
            enabled: true,
            context: TelemetryContext::with_i_key(config.i_key().to_string()),
            channel: InMemoryChannel::new(&config),
        }
    }
}

impl<C> TelemetryClient<C>
where
    C: TelemetryChannel,
{
    /// Determines whether this client is enabled and will accept telemetry.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// assert!(client.is_enabled(), true);
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
    /// assert_eq!(client.is_enabled(), true);
    ///
    /// client.enabled(false);
    /// assert_eq!(client.is_enabled(), false);
    /// ```
    pub fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Logs a user action with the specified name.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use appinsights::TelemetryClient;
    /// # let client = TelemetryClient::new("<instrumentation key>".to_string());
    /// client.track_event("app is running".to_string());
    /// ```
    pub fn track_event(&self, name: String) {
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
    /// client.track_trace("Unable to connect to a gateway".to_string(), SeverityLevel::Warning);
    /// ```
    pub fn track_trace(&self, message: String, severity: SeverityLevel) {
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
    /// client.track_metric("gateway_latency_ms".to_string(), 113.0);
    /// ```    
    pub fn track_metric(&self, name: String, value: f64) {
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
    /// client.track_request(Method::GET, uri, Duration::from_millis(100), "200".to_string());
    /// ```
    pub fn track_request(&self, method: Method, uri: Uri, duration: Duration, response_code: String) {
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
    ///     "GET https://api.github.com/dmolokanov/appinsights-rs".to_string(),
    ///     "HTTP".to_string(),
    ///     "api.github.com".to_string(),
    ///     true
    /// );
    /// ```
    pub fn track_remote_dependency(&self, name: String, dependency_type: String, target: String, success: bool) {
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
    ///     "GET https://api.github.com/dmolokanov/appinsights-rs".to_string(),
    ///     Duration::from_millis(100),
    ///     true
    /// );
    /// ```
    pub fn track_availability(&self, name: String, duration: Duration, success: bool) {
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
    /// let mut telemetry = AggregateMetricTelemetry::new("device_message_latency_per_min".into());
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

    /// Flushes and tears down the submission flow and closes internal channels.
    /// It block current thread until all pending telemetry items have been submitted and it is safe to
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
    ///     client.track_event("app is running".to_string());
    /// }
    ///
    /// // wait until pending telemetry is sent at most once and tear down submission flow
    /// client.close_channel();
    ///
    /// // unable to sent any telemetry after client closes its channel
    /// // client.track_event("app is stopped".to_string());
    /// ```
    pub fn close_channel(self) {
        let mut channel = self.channel;
        channel.close()
    }

    /// Forces all pending telemetry items to be submitted. The current thread will not be blocked.
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
    ///     client.track_event("app is running".to_string());
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
}

impl From<(TelemetryConfig, TelemetryContext)> for TelemetryClient<InMemoryChannel> {
    fn from((config, context): (TelemetryConfig, TelemetryContext)) -> Self {
        Self {
            enabled: true,
            context,
            channel: InMemoryChannel::new(&config),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use chrono::{DateTime, Utc};

    use super::*;

    #[test]
    fn it_enabled_by_default() {
        let client = TelemetryClient::new("key".into());
        assert_eq!(client.is_enabled(), true)
    }

    #[test]
    fn it_disables_telemetry() {
        let mut client = TelemetryClient::new("key".into());

        client.enabled(false);

        assert_eq!(client.is_enabled(), false)
    }

    #[test]
    fn it_submits_telemetry() {
        let client = create_client();

        client.track(TestTelemetry {});

        let events = client.channel.events.borrow();
        assert_eq!(events.len(), 1)
    }

    #[test]
    fn it_swallows_telemetry_when_disabled() {
        let mut client = create_client();
        client.enabled(false);

        client.track(TestTelemetry {});

        let events = client.channel.events.borrow();
        assert!(events.is_empty())
    }

    fn create_client() -> TelemetryClient<TestChannel> {
        TelemetryClient {
            enabled: true,
            context: TelemetryContext::with_i_key("instrumentation key".to_string()),
            channel: TestChannel {
                events: RefCell::new(Vec::new()),
            },
        }
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
        events: RefCell<Vec<Envelope>>,
    }

    impl TelemetryChannel for TestChannel {
        fn send(&self, envelop: Envelope) {
            self.events.borrow_mut().push(envelop);
        }

        fn flush(&self) {
            unimplemented!()
        }

        fn close(&mut self) {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod integration_tests;
