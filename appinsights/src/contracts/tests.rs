use crate::contracts::*;
use crate::telemetry::{ContextTags, Measurements, Properties};
use chrono::{DateTime, SecondsFormat, TimeZone, Utc};
use serde_json::json;

#[test]
fn it_converts_event_data_to_json() {
    let envelope = EnvelopeBuilder::new(
        "letter",
        Utc.ymd(2019, 8, 11)
            .and_hms(1, 2, 3)
            .to_rfc3339_opts(SecondsFormat::Millis, true),
    )
    .i_key("instrumentation")
    .tags(ContextTags::default())
    .flags(0)
    .data(Base::Data(Data::EventData(
        EventDataBuilder::new("message received")
            .properties(Properties::default())
            .measurements(Measurements::default())
            .build(),
    )))
    .build();

    let actual = serde_json::to_value(&envelope).unwrap();
    let expected = json!({
        "ver": 1,
        "name": "letter",
        "time": String::from("2019-08-11T01:02:03.000Z"),
        "iKey": "instrumentation",
        "tags": {},
        "seq": null,
        "flags": 0,
        "sampleRate": 100.0,
        "data": {
            "baseType": "EventData",
            "baseData": {
                "ver": 2,
                "name": "message received",
                "measurements": {},
                "properties": {},

            }
        }

    });

    assert_eq!(actual.to_string(), expected.to_string())
}
