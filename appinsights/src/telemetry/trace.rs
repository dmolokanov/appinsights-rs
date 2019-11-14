use chrono::{DateTime, SecondsFormat, Utc};

use crate::context::TelemetryContext;
use crate::contracts::{SeverityLevel as ContractsSeverityLevel, *};
use crate::telemetry::{ContextTags, Properties, Telemetry};

// Represents printf-like trace statements that can be text searched.
pub struct TraceTelemetry {
    /// A trace message.
    message: String,

    // Severity level.
    severity: SeverityLevel,

    /// The time stamp when this telemetry was measured.
    timestamp: DateTime<Utc>,

    /// Custom properties.
    properties: Properties,

    /// Telemetry context containing extra, optional tags.
    tags: ContextTags,
}

impl TraceTelemetry {
    /// Creates an event telemetry item with specified name.
    pub fn new(timestamp: DateTime<Utc>, message: &str, severity: SeverityLevel) -> Self {
        Self {
            message: message.into(),
            severity,
            timestamp,
            properties: Default::default(),
            tags: Default::default(),
        }
    }
}

impl Telemetry for TraceTelemetry {
    /// Returns the time when this telemetry was measured.
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// Returns custom properties to submit with the telemetry item.
    fn properties(&self) -> &Properties {
        &self.properties
    }

    /// Returns mutable reference to custom properties.
    fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Returns context data containing extra, optional tags. Overrides values found on client telemetry context.
    fn tags(&self) -> &ContextTags {
        &self.tags
    }

    /// Returns mutable reference to custom tags.
    fn tags_mut(&mut self) -> &mut ContextTags {
        &mut self.tags
    }
}

impl From<TraceTelemetry> for Data {
    fn from(telemetry: TraceTelemetry) -> Self {
        Data::MessageData(
            MessageDataBuilder::new(telemetry.message)
                .severity_level(telemetry.severity.into())
                .properties(telemetry.properties)
                .build(),
        )
    }
}

impl From<(TelemetryContext, TraceTelemetry)> for Envelope {
    fn from((context, telemetry): (TelemetryContext, TraceTelemetry)) -> Self {
        let data = Data::MessageData(
            MessageDataBuilder::new(telemetry.message)
                .severity_level(telemetry.severity.into())
                .properties(Properties::combine(context.properties, telemetry.properties))
                .build(),
        );

        let envelope_name = data.envelope_name(&context.normalized_i_key);
        let timestamp = telemetry.timestamp.to_rfc3339_opts(SecondsFormat::Millis, true);

        EnvelopeBuilder::new(envelope_name, timestamp)
            .data(Base::Data(data))
            .i_key(context.i_key)
            .tags(ContextTags::combine(context.tags, telemetry.tags))
            .build()
    }
}

/// Defines the level of severity for the event.
pub enum SeverityLevel {
    Verbose,
    Information,
    Warning,
    Error,
    Critical,
}

impl From<SeverityLevel> for ContractsSeverityLevel {
    fn from(severity: SeverityLevel) -> Self {
        match severity {
            SeverityLevel::Verbose => ContractsSeverityLevel::Verbose,
            SeverityLevel::Information => ContractsSeverityLevel::Information,
            SeverityLevel::Warning => ContractsSeverityLevel::Warning,
            SeverityLevel::Error => ContractsSeverityLevel::Error,
            SeverityLevel::Critical => ContractsSeverityLevel::Critical,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::TimeZone;

    use super::*;

    #[test]
    fn it_overrides_properties_from_context() {
        let mut context = TelemetryContext::new("instrumentation".into());
        context.properties_mut().insert("test".into(), "ok".into());
        context.properties_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = TraceTelemetry::new(
            Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 600),
            "message".into(),
            SeverityLevel::Information,
        );
        telemetry.properties_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.Message".into(),
            "2019-01-02T03:04:05.600Z".into(),
        )
        .data(Base::Data(Data::MessageData(
            MessageDataBuilder::new("message".into())
                .severity_level(crate::contracts::SeverityLevel::Information)
                .properties({
                    let mut properties = BTreeMap::default();
                    properties.insert("test".into(), "ok".into());
                    properties.insert("no-write".into(), "ok".into());
                    properties
                })
                .build(),
        )))
        .i_key("instrumentation".into())
        .tags(BTreeMap::default())
        .build();

        assert_eq!(envelop, expected)
    }

    #[test]
    fn it_overrides_tags_from_context() {
        let mut context = TelemetryContext::new("instrumentation".into());
        context.tags_mut().insert("test".into(), "ok".into());
        context.tags_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = TraceTelemetry::new(
            Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 600),
            "message".into(),
            SeverityLevel::Information,
        );
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.Message".into(),
            "2019-01-02T03:04:05.600Z".into(),
        )
        .data(Base::Data(Data::MessageData(
            MessageDataBuilder::new("message".into())
                .severity_level(crate::contracts::SeverityLevel::Information)
                .properties(BTreeMap::default())
                .build(),
        )))
        .i_key("instrumentation".into())
        .tags({
            let mut tags = BTreeMap::default();
            tags.insert("test".into(), "ok".into());
            tags.insert("no-write".into(), "ok".into());
            tags
        })
        .build();

        assert_eq!(envelop, expected)
    }
}
