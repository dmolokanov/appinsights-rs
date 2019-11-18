use chrono::{DateTime, Utc};
use http::header::RETRY_AFTER;
use http::StatusCode;
use reqwest::blocking::Client;
use std::time::Duration;

use crate::contracts::Envelope;
use crate::transmitter::Transmission;
use crate::Result;

pub struct Transmitter {
    url: String,
    client: Client,
}

impl Transmitter {
    pub fn new(url: &str) -> Self {
        let client = Client::new();
        Self {
            url: url.into(),
            client,
        }
    }

    pub fn transmit(&self, items: &Vec<Envelope>) -> Result<Transmission> {
        //        let payload = serde_json::to_string(&items)?;
        //
        //        let response = self.client.post(&self.url).body(payload).send()?;
        //
        //        let status_code = response.status();
        //
        //        let retry_after = if let Some(retry_after) = response.headers().get(RETRY_AFTER) {
        //            let retry_after = retry_after.to_str()?;
        //            Some(DateTime::parse_from_rfc2822(retry_after)?.with_timezone(&Utc))
        //        } else {
        //            None
        //        };
        //
        //        Ok(Transmission::new(status_code, retry_after, response.json()?)
        std::thread::sleep(Duration::from_secs(1));

        Ok(Transmission::new(
            StatusCode::OK,
            None,
            serde_json::from_value(
                serde_json::json!({"itemsReceived": items.len(), "itemsAccepted": items.len(), "errors":[] }),
            )
            .unwrap(),
        ))
    }
}
