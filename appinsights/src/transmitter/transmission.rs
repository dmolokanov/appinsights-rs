use crate::contracts::Envelope;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Deserialize;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Transmission {
    status_code: StatusCode,
    retry_after: Option<DateTime<Utc>>,
    can_retry_item_indices: Option<BTreeSet<usize>>,
    success: bool,
}

/// Describes the result of sending telemetry events to the server.
impl Transmission {
    /// Creates a new instance of object that describes the result of sending telemetry events to the server.
    pub fn new(status_code: StatusCode, retry_after: Option<DateTime<Utc>>, response: TransmissionResponse) -> Self {
        let can_retry_item_indices = if status_code == StatusCode::PARTIAL_CONTENT {
            let indices = response
                .errors
                .iter()
                .filter_map(|error| if error.can_retry() { Some(error.index) } else { None })
                .collect();
            Some(indices)
        } else {
            None
        };

        let success = status_code == StatusCode::OK
            || status_code == StatusCode::PARTIAL_CONTENT && response.items_received == response.items_accepted;

        Self {
            status_code,
            retry_after,
            can_retry_item_indices,
            success,
        }
    }

    /// Returns true when all telemetry events were accepted; false otherwise.
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Returns true when client should retry an attempt to re-send some telemetry events back to the server.
    pub fn can_retry(&self) -> bool {
        if self.is_success() {
            false
        } else {
            self.status_code == StatusCode::PARTIAL_CONTENT
                || self.status_code == StatusCode::REQUEST_TIMEOUT
                || self.status_code == StatusCode::INTERNAL_SERVER_ERROR
                || self.status_code == StatusCode::SERVICE_UNAVAILABLE
                || self.status_code == StatusCode::TOO_MANY_REQUESTS
        }
    }

    pub fn can_retry_item(&self, index: usize) -> bool {
        if let Some(indices) = &self.can_retry_item_indices {
            indices.contains(&index)
        } else {
            true
        }
    }

    //    /// Filters out those telemetry items that can be re-send back to the server.
    //    pub fn retry_items(&self, mut items: Vec<Envelope>) -> Vec<Envelope> {
    //        if self.status_code == StatusCode::PARTIAL_CONTENT {
    //            let indices: BTreeSet<_> = self
    //                .response
    //                .errors
    //                .iter()
    //                .filter_map(|error| if error.can_retry() { Some(error.index) } else { None })
    //                .collect();
    //
    //            items
    //                .drain(..)
    //                .into_iter()
    //                .enumerate()
    //                .filter_map(|(i, envelope)| if indices.contains(&i) { Some(envelope) } else { None })
    //                .collect()
    //        } else {
    //            items
    //        }
    //    }
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
    index: usize,
    status_code: u16,
    message: String,
}

impl TransmissionItem {
    pub fn can_retry(&self) -> bool {
        self.status_code == StatusCode::PARTIAL_CONTENT
            || self.status_code == StatusCode::REQUEST_TIMEOUT
            || self.status_code == StatusCode::INTERNAL_SERVER_ERROR
            || self.status_code == StatusCode::SERVICE_UNAVAILABLE
            || self.status_code == StatusCode::TOO_MANY_REQUESTS
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use test_case::test_case;

    use super::*;

    #[test]
    fn it_success_when_status_is_ok() {
        let transmission = Transmission::new(
            StatusCode::OK,
            None,
            TransmissionResponse {
                items_received: 0,
                items_accepted: 0,
                errors: Default::default(),
            },
        );

        assert!(transmission.is_success());
    }

    #[test]
    fn it_success_when_status_is_partial_content_and_all_item_accepted() {
        let transmission = Transmission::new(
            StatusCode::PARTIAL_CONTENT,
            None,
            TransmissionResponse {
                items_received: 3,
                items_accepted: 3,
                errors: Default::default(),
            },
        );

        assert!(transmission.is_success());
    }

    #[test_case(StatusCode::PARTIAL_CONTENT, TransmissionResponse { items_received: 2, items_accepted: 1, errors: Default::default() }; "when partial and not all items accepted")]
    #[test_case(StatusCode::BAD_REQUEST, TransmissionResponse { items_received: 0, items_accepted: 0, errors: Default::default() }; "when bad request")]
    fn it_is_not_success(status_code: StatusCode, response: TransmissionResponse) {
        let transmission = Transmission::new(status_code, None, response);

        assert!(!transmission.is_success());
    }

    #[test_case(StatusCode::PARTIAL_CONTENT; "when partial content and not all items accepted")]
    #[test_case(StatusCode::REQUEST_TIMEOUT; "when request timeout")]
    #[test_case(StatusCode::INTERNAL_SERVER_ERROR; "when internal server error")]
    #[test_case(StatusCode::SERVICE_UNAVAILABLE; "when service unavailable")]
    #[test_case(StatusCode::TOO_MANY_REQUESTS; "when too many requests")]
    fn it_can_retry(status_code: StatusCode) {
        let transmission = Transmission::new(
            status_code,
            None,
            TransmissionResponse {
                items_received: 1,
                items_accepted: 0,
                errors: Default::default(),
            },
        );

        assert!(transmission.can_retry())
    }
}
