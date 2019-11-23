use std::time::Duration;

/// Encapsulates retry logic for submit telemetry items operation.
#[derive(Default, Debug)]
pub struct Retry(Vec<Duration>);

impl Retry {
    pub fn exponential() -> Self {
        let timeouts = vec![Duration::from_secs(16), Duration::from_secs(4), Duration::from_secs(2)];
        Self(timeouts)
    }

    pub fn once() -> Self {
        Self::default()
    }

    pub fn next(&mut self) -> Option<Duration> {
        self.0.pop()
    }
}
