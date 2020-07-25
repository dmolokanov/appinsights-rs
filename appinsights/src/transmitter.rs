use chrono::{DateTime, Utc};
use http::{header::RETRY_AFTER, StatusCode};
use log::debug;
use reqwest::blocking::Client;

use crate::{
    contracts::{Envelope, Transmission, TransmissionItem},
    Result,
};

#[derive(Debug, PartialEq)]
pub enum Response {
    Success,
    Retry(Vec<Envelope>),
    Throttled(DateTime<Utc>, Vec<Envelope>),
    NoRetry,
}

/// Sends telemetry items to the server.
pub struct Transmitter {
    url: String,
    client: Client,
}

impl Transmitter {
    /// Creates a new instance of telemetry items sender.
    pub fn new(url: &str) -> Self {
        let client = Client::new();
        Self {
            url: url.into(),
            client,
        }
    }

    // TODO get rid of to_vec() calls
    /// Sends a telemetry items to the server.
    pub fn send(&self, items: &mut Vec<Envelope>) -> Result<Response> {
        let payload = serde_json::to_string(&items)?;

        let response = self.client.post(&self.url).body(payload).send()?;
        let response = match response.status() {
            StatusCode::OK => {
                debug!("Successfully sent {} items", items.len());
                Response::Success
            }
            StatusCode::PARTIAL_CONTENT => {
                let content: Transmission = response.json()?;
                let log_prefix = format!(
                    "Successfully sent {}/{} telemetry items",
                    content.items_accepted, content.items_received
                );
                if content.items_received == content.items_accepted {
                    debug!("{}", log_prefix);
                    Response::Success
                } else {
                    let retry_items = retry_items(items, content);
                    if retry_items.is_empty() {
                        debug!("{}. Nothing to re-send", log_prefix);
                        Response::NoRetry
                    } else {
                        debug!("{}. Retry sending {} items", log_prefix, retry_items.len());
                        Response::Retry(retry_items)
                    }
                }
            }
            StatusCode::TOO_MANY_REQUESTS | StatusCode::REQUEST_TIMEOUT => {
                let retry_after = response.headers().get(RETRY_AFTER).cloned();

                let items = if let Ok(content) = response.json::<Transmission>() {
                    retry_items(items, content)
                } else {
                    items.to_vec()
                };

                if let Some(retry_after) = retry_after {
                    let retry_after = retry_after.to_str()?;
                    let retry_after = DateTime::parse_from_rfc2822(retry_after)?.with_timezone(&Utc);
                    debug!(
                        "Some items were discarded. Retry sending {} items after {}",
                        items.len(),
                        retry_after
                    );
                    Response::Throttled(retry_after, items)
                } else {
                    debug!("Some items were discarded. Retry sending {} items", items.len());
                    Response::Retry(items)
                }
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                debug!("Service unavailable. Retry sending {} items", items.len());
                Response::Retry(items.to_vec())
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                if let Ok(content) = response.json::<Transmission>() {
                    let retry_items = retry_items(items, content);
                    if retry_items.is_empty() {
                        debug!("Service error. Nothing to re-send");
                        Response::NoRetry
                    } else {
                        debug!("Service error. Retry sending {} items", retry_items.len());
                        Response::Retry(retry_items)
                    }
                } else {
                    debug!("Service error. Retry sending {} items", items.len());
                    Response::Retry(items.to_vec())
                }
            }
            _ => {
                debug!("Unknown status: {}. Nothing to re-send", response.status());
                Response::NoRetry
            }
        };

        Ok(response)
    }
}

/// Filters out those telemetry items that cannot be re-sent.
fn retry_items(items: &mut Vec<Envelope>, content: Transmission) -> Vec<Envelope> {
    let mut retry_items = Vec::default();
    for error in &content.errors {
        if can_retry_item(error) {
            retry_items.push(items.remove(error.index - retry_items.len()));
        }
    }

    retry_items
}

/// Determines that a telemetry item can be re-send corresponding to this submission status
/// descriptor.
fn can_retry_item(item: &TransmissionItem) -> bool {
    item.status_code == StatusCode::PARTIAL_CONTENT
        || item.status_code == StatusCode::REQUEST_TIMEOUT
        || item.status_code == StatusCode::INTERNAL_SERVER_ERROR
        || item.status_code == StatusCode::SERVICE_UNAVAILABLE
        || item.status_code == StatusCode::TOO_MANY_REQUESTS
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use http::StatusCode;
    use mockito::mock;
    use serde_json::{json, Value};
    use test_case::test_case;

    use super::*;

    #[test_case(items(), StatusCode::OK, None, Some(all_accepted()), Response::Success; "success")]
    #[test_case(items(), StatusCode::PARTIAL_CONTENT, None, Some(partial_some_retries()), Response::Retry(retry_items()); "partial. resend some items")]
    #[test_case(items(), StatusCode::PARTIAL_CONTENT, None, Some(partial_no_retries()), Response::NoRetry; "partial. nothing to resend")]
    #[test_case(items(), StatusCode::PARTIAL_CONTENT, None, Some(none_accepted()), Response::Retry(items()); "partial. resend everything")]
    #[test_case(items(), StatusCode::PARTIAL_CONTENT, None, Some(all_accepted()), Response::Success; "partial. everything accepted")]
    #[test_case(items(), StatusCode::BAD_REQUEST, None, None, Response::NoRetry; "bad request. no retry")]
    #[test_case(items(), StatusCode::REQUEST_TIMEOUT, None, None, Response::Retry(items()); "timeout. resend everything")]
    #[test_case(items(), StatusCode::REQUEST_TIMEOUT, Some(retry_after_str()), None, Response::Throttled(retry_after(), items()); "timeout. throttled")]
    #[test_case(items(), StatusCode::TOO_MANY_REQUESTS, None, None,Response::Retry(items()); "too many requests. no retry-after. resend everything")]
    #[test_case(items(), StatusCode::TOO_MANY_REQUESTS, Some(retry_after_str()), None, Response::Throttled(retry_after(), items()); "too many requests. retry-after. throttled")]
    #[test_case(items(), StatusCode::INTERNAL_SERVER_ERROR, None, None, Response::Retry(items()); "server error. resend everything")]
    #[test_case(items(), StatusCode::SERVICE_UNAVAILABLE, None, None, Response::Retry(items()); "service unavailable. resend everything")]
    #[test_case(items(), StatusCode::UNAUTHORIZED, None, None, Response::NoRetry; "unauthorized. no retry")]
    #[test_case(items(), StatusCode::REQUEST_TIMEOUT, None, Some(partial_some_retries()), Response::Retry(retry_items()); "timeout. resend some items")]
    #[test_case(items(), StatusCode::INTERNAL_SERVER_ERROR, None, Some(partial_some_retries()), Response::Retry(retry_items()); "server error. resend some items")]
    fn it_sends_telemetry_and_handles_server_response(
        mut items: Vec<Envelope>,
        status_code: StatusCode,
        retry_after: Option<&str>,
        body: Option<Value>,
        expected: Response,
    ) {
        let mut _server = mock("POST", "/track").with_status(status_code.as_u16() as usize);
        if let Some(retry_after) = retry_after {
            _server = _server.with_header("Retry-After", retry_after);
        }
        if let Some(body) = body {
            _server = _server.with_body(body.to_string());
        }
        _server = _server.create();
        let url = mockito::server_url();

        let transmitter = Transmitter::new(&format!("{}/track", url));

        let response = transmitter.send(&mut items).unwrap();

        assert_eq!(response, expected)
    }

    fn partial_no_retries() -> Value {
        json!({
            "itemsAccepted": 3,
            "itemsReceived": 5,
            "errors": [
                {
                    "index": 2,
                    "statusCode": 400,
                    "message": "Bad 1"
                },
                {
                    "index": 4,
                    "statusCode": 400,
                    "message": "Bad 2"
                },
            ],
        })
    }

    fn partial_some_retries() -> Value {
        json!({
            "itemsAccepted": 2,
            "itemsReceived": 5,
            "errors": [
                {
                    "index": 2,
                    "statusCode": 400,
                    "message": "Bad 1"
                },
                {
                    "index": 4,
                    "statusCode": 408,
                    "message": "OK Later"
                },
            ],
        })
    }

    fn none_accepted() -> Value {
        json!({
            "itemsAccepted": 0,
            "itemsReceived": 5,
            "errors": [
                {
                    "index": 0,
                    "statusCode": 500,
                    "message": "Bad 1"
                },
                {
                    "index": 1,
                    "statusCode": 500,
                    "message": "Bad 2"
                },
                {
                    "index": 2,
                    "statusCode": 500,
                    "message": "Bad 3"
                },
                {
                    "index": 3,
                    "statusCode": 500,
                    "message": "Bad 4"
                },
                {
                    "index": 4,
                    "statusCode": 500,
                    "message": "Bad 5"
                },
            ],
        })
    }

    fn all_accepted() -> Value {
        json!({
            "itemsAccepted": 5,
            "itemsReceived": 5,
            "errors": [],
        })
    }

    fn retry_after_str() -> &'static str {
        "Wed, 09 Aug 2017 23:43:57 GMT"
    }

    fn retry_after() -> DateTime<Utc> {
        Utc.ymd(2017, 8, 9).and_hms(23, 43, 57)
    }

    fn items() -> Vec<Envelope> {
        (0..5)
            .map(|i| Envelope {
                name: format!("event {}", i),
                ..Envelope::default()
            })
            .collect()
    }

    fn retry_items() -> Vec<Envelope> {
        vec![Envelope {
            name: "event 4".into(),
            ..Envelope::default()
        }]
    }
}
