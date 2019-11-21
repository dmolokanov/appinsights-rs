use std::time::Duration;

use http::{Method, Uri};

use crate::channel::{InMemoryChannel, TelemetryChannel};
use crate::context::TelemetryContext;
use crate::contracts::Envelope;
use crate::telemetry::*;
use crate::Config;
use crate::Result;

/// Application Insights telemetry client provides an interface to track telemetry items.
pub struct TelemetryClient<C> {
    enabled: bool,
    context: TelemetryContext,
    channel: C,
}

impl TelemetryClient<InMemoryChannel> {
    /// Creates a new telemetry client that submits telemetry with specified instrumentation key.
    pub fn new(i_key: String) -> Self {
        Self::from_config(Config::new(i_key))
    }
}

impl TelemetryClient<InMemoryChannel> {
    /// Creates a new telemetry client configured with specified configuration.
    pub fn from_config(config: Config) -> Self {
        Self {
            enabled: true,
            context: TelemetryContext::new(config.i_key().to_string()),
            channel: InMemoryChannel::new(&config),
        }
    }
}

impl<C> TelemetryClient<C>
where
    C: TelemetryChannel,
{
    /// Determines whether this client is enabled and will accept telemetry.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables or disables telemetry client. When disabled, telemetry is silently swallowed by the client. Defaults to enabled.
    pub fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns the telemetry channel used to submit data to the server.
    pub fn channel(&self) -> &C {
        &self.channel
    }

    /// Logs a user action with the specified name.
    pub fn track_event(&self, name: String) -> Result<()> {
        let event = EventTelemetry::new(name);
        self.track(event)
    }

    /// Logs a trace message with a specified severity level.
    pub fn track_trace(&self, message: String, severity: SeverityLevel) -> Result<()> {
        let event = TraceTelemetry::new(message, severity);
        self.track(event)
    }

    /// Logs a numeric value that is not specified with a specific event.
    /// Typically used to send regular reports of performance indicators.
    pub fn track_metric(&self, name: String, value: f64) -> Result<()> {
        let event = MetricTelemetry::new(name, value);
        self.track(event)
    }

    /// Logs an HTTP request with the specified method, URL, duration and response code.
    pub fn track_request(&self, method: Method, uri: Uri, duration: Duration, response_code: String) -> Result<()> {
        let event = RequestTelemetry::new(method, uri, duration, response_code);
        self.track(event)
    }

    /// Logs a dependency with the specified name, type, target, and success status.
    pub fn track_remote_dependency(
        &self,
        name: String,
        dependency_type: String,
        target: String,
        success: bool,
    ) -> Result<()> {
        let event = RemoteDependencyTelemetry::new(name, dependency_type, Default::default(), target, success);
        self.track(event)
    }

    /// Logs an availability test result with the specified test name, duration, and success status.
    pub fn track_availability(&self, name: String, duration: Duration, success: bool) -> Result<()> {
        let event = AvailabilityTelemetry::new(name, duration, success);
        self.track(event)
    }

    /// Submits a specific telemetry event.
    pub fn track<E>(&self, event: E) -> Result<()>
    where
        E: Telemetry,
        (TelemetryContext, E): Into<Envelope>,
    {
        if self.is_enabled() {
            let envelop = (self.context.clone(), event).into();
            self.channel.send(envelop)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use chrono::{DateTime, SecondsFormat, Utc};

    use super::*;
    use crate::contracts::EnvelopeBuilder;
    use crate::time;
    use crate::Result;

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
            context: TelemetryContext::new("instrumentation key".to_string()),
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
            EnvelopeBuilder::new("test", time::now().to_rfc3339_opts(SecondsFormat::Millis, true)).build()
        }
    }

    struct TestChannel {
        events: RefCell<Vec<Envelope>>,
    }

    impl TelemetryChannel for TestChannel {
        fn send(&self, envelop: Envelope) -> Result<()> {
            self.events.borrow_mut().push(envelop);
            Ok(())
        }

        fn flush(&self) -> Result<()> {
            unimplemented!()
        }

        fn close(&self) -> Result<()> {
            unimplemented!()
        }
    }
}
