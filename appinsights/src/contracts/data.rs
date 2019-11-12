use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Data struct to contain both B and C sections.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "base_type", content = "base_data")]
pub enum Data {
    AvailabilityData(AvailabilityData),
    EventData(EventData),
    ExceptionData(ExceptionData),
    MessageData(MessageData),
    MetricData(MetricData),
    PageViewData(PageViewData),
    RemoteDependencyData(RemoteDependencyData),
    RequestData(RequestData),
}

impl Data {
    pub fn envelope_name(&self, key: &str) -> String {
        let name = match self {
            Data::AvailabilityData(_) => "Availability",
            Data::EventData(_) => "Event",
            Data::ExceptionData(_) => "Exception",
            Data::MessageData(_) => "Message",
            Data::MetricData(_) => "Metric",
            Data::PageViewData(_) => "PageView",
            Data::RemoteDependencyData(_) => "RemoteDependency",
            Data::RequestData(_) => "Request",
        };

        if key.is_empty() {
            format!("Microsoft.ApplicationInsights.{}.{}", key, name)
        } else {
            format!("Microsoft.ApplicationInsights.{}", name)
        }
    }
}
