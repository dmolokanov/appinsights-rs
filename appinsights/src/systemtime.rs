use chrono::{DateTime, Utc};

/// A trait to provide current date.
pub trait SystemTime {
    /// Returns a DataTime which corresponds a current date.
    fn now() -> DateTime<Utc>;
}

impl SystemTime for Utc {
    /// Returns a DataTime which corresponds a current date.
    fn now() -> DateTime<Utc> {
        Utc::now()
    }
}
