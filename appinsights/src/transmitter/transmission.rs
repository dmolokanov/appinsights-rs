use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Deserialize;

#[derive(Debug)]
pub struct Transmission {
    status_code: StatusCode,
    retry_after: Option<DateTime<Utc>>,
    response: TransmissionResponse,
}

impl Transmission {
    pub fn new(status_code: StatusCode, retry_after: Option<DateTime<Utc>>, response: TransmissionResponse) -> Self {
        Self {
            status_code,
            retry_after,
            response,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransmissionResponse {
    items_received: u32,
    items_accepted: u32,
    errors: Vec<TransmissionItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransmissionItem {
    index: u32,
    status_code: u16,
    message: String,
}
