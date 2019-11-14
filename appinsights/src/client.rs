use std::marker::PhantomData;

use chrono::Utc;

use crate::channel::{InMemoryChannel, TelemetryChannel};
use crate::context::TelemetryContext;
use crate::contracts::Envelope;
use crate::telemetry::{EventTelemetry, SeverityLevel, Telemetry, TraceTelemetry};
use crate::{Config, SystemTime};

/// Application Insights telemetry client provides an interface to track telemetry items.
pub struct TelemetryClient<C, T> {
    enabled: bool,
    context: TelemetryContext,
    channel: C,
    phantom: PhantomData<T>,
}

impl TelemetryClient<InMemoryChannel, Utc> {
    /// Creates a new telemetry client that submits telemetry with specified instrumentation key.
    pub fn new(i_key: String) -> Self {
        Self::from_config(Config::new(i_key))
    }
}

impl<T> TelemetryClient<InMemoryChannel, T> {
    /// Creates a new telemetry client configured with specified configuration.
    pub fn from_config(config: Config) -> Self {
        Self {
            enabled: true,
            context: TelemetryContext::new(config.i_key().to_string()),
            channel: InMemoryChannel::new(&config),
            phantom: PhantomData,
        }
    }
}

impl<C, T> TelemetryClient<C, T>
where
    C: TelemetryChannel,
    T: SystemTime,
{
    /// Determines whether this client is enabled and will accept telemetry.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables or disables telemetry client. When disabled, telemetry is silently swallowed by the client. Defaults to enabled.
    pub fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Logs a user action with the specified name.
    pub fn track_event(&self, name: &str) {
        let event = EventTelemetry::new(T::now(), name);
        self.track(event)
    }

    /// Logs a trace message with a specified severity level.
    pub fn track_trace(&self, message: &str, severity: SeverityLevel) {
        let event = TraceTelemetry::new(T::now(), message, severity);
        self.track(event)
    }

    /// Submits a specific telemetry event.
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
}

#[cfg(test)]
mod tests {
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

    //    #[test]
    //    #[ignore]
    //    fn it_submits_telemetry() {
    //        let client = create_client();
    //
    //        client.track(TestTelemetry {});
    //
    //        let events = client.channel.events.borrow();
    //        assert_eq!(events.len(), 1)
    //    }
    //
    //    #[test]
    //    fn it_swallows_telemetry_when_disabled() {
    //        let mut client = create_client();
    //        client.enabled(false);
    //
    //        client.track(TestTelemetry {});
    //
    //        let events = client.channel.events.borrow();
    //        assert!(events.is_empty())
    //    }
    //
    //    fn create_client() -> TelemetryClient<TestChannel> {
    //        let config = Config::new("instrumentation key".into());
    //
    //        TelemetryClient {
    //            enabled: true,
    //            context: TelemetryContext::from(&config),
    //            channel: TestChannel {
    //                events: RefCell::new(Vec::new()),
    //            },
    //        }
    //    }
    //
    //    struct TestTelemetry {}
    //
    //    impl Telemetry for TestTelemetry {
    //        fn timestamp(&self) -> DateTime<Utc> {
    //            unimplemented!()
    //        }
    //
    //        fn properties(&self) -> &BTreeMap<String, String, RandomState> {
    //            unimplemented!()
    //        }
    //
    //        fn measurements(&self) -> Option<&BTreeMap<String, f64, RandomState>> {
    //            unimplemented!()
    //        }
    //
    //        fn tags(&self) -> &BTreeMap<String, String, RandomState> {
    //            unimplemented!()
    //        }
    //    }
    //
    //    #[derive(Clone)]
    //    struct TestData;
    //
    //    impl From<TestTelemetry> for Data {
    //        fn from(_: TestTelemetry) -> Self {
    //            unimplemented!()
    //        }
    //    }
    //
    //    struct TestChannel {
    //        events: RefCell<Vec<Envelope>>,
    //    }
    //
    //    impl TelemetryChannel for TestChannel {
    //        fn send(&self, envelop: Envelope) -> Result<()> {
    //            self.events.borrow_mut().push(envelop);
    //            Ok(())
    //        }
    //    }
}
