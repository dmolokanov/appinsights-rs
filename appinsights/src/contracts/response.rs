use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transmission {
    pub items_received: usize,
    pub items_accepted: usize,
    pub errors: Vec<TransmissionItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransmissionItem {
    pub index: usize,
    pub status_code: u16,
    pub message: String,
}
