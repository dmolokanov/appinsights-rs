pub use imp::*;

use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::time::Duration as StdDuration;

#[cfg(not(test))]
mod imp {
    use chrono::{DateTime, Utc};

    /// Returns a DateTime which corresponds to a current date.
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(test)]
mod imp {
    use std::cell::RefCell;

    use chrono::{DateTime, Utc};

    thread_local!(static NOW: RefCell<Option<DateTime<Utc>>> = RefCell::new(None));

    /// Returns a DateTime which corresponds to a current date or the value user set in advance.
    pub fn now() -> DateTime<Utc> {
        NOW.with(|ts| if let Some(now) = *ts.borrow() { now } else { Utc::now() })
    }

    /// Sets known DateTime value as now to assert test against it.
    pub fn set(now: DateTime<Utc>) {
        NOW.with(|ts| *ts.borrow_mut() = Some(now))
    }

    /// Resets pre-defined DateTime value to use Utc::now() instead.
    pub fn reset() {
        NOW.with(|ts| *ts.borrow_mut() = None)
    }
}

/// Provides dotnet duration aware formatting rules.
pub struct Duration(StdDuration);

impl From<StdDuration> for Duration {
    fn from(duration: StdDuration) -> Self {
        Duration(duration)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let nanoseconds = self.0.as_nanos();
        let ticks = nanoseconds / 100 % 10_000_000;
        let total_seconds = nanoseconds / 1_000_000_000;
        let seconds = total_seconds % 60;
        let minutes = total_seconds / 60 % 60;
        let hours = total_seconds / 3600 % 24;
        let days = total_seconds / 86400;

        write!(
            f,
            "{}.{:0>2}:{:0>2}:{:0>2}.{:0>7}",
            days, hours, minutes, seconds, ticks
        )
    }
}

impl Deref for Duration {
    type Target = StdDuration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use test_case::test_case;

    use super::*;

    #[test_case(StdDuration::from_secs(3600).into(),  "0.01:00:00.0000000"    ; "hour")]
    #[test_case(StdDuration::from_secs(60).into(),    "0.00:01:00.0000000"    ; "minute")]
    #[test_case(StdDuration::from_secs(1).into(),     "0.00:00:01.0000000"    ; "second")]
    #[test_case(StdDuration::from_millis(1).into(),   "0.00:00:00.0010000"    ; "millisecond")]
    #[test_case(StdDuration::from_nanos(100).into(),  "0.00:00:00.0000001"    ; "tick")]
    #[test_case((Utc.ymd(2019, 1, 3).and_hms(1, 2, 3) - Utc.ymd(2019, 1, 1).and_hms(0, 0, 0)).to_std().unwrap().into(), "2.01:02:03.0000000"    ; "custom")]
    fn it_converts_duration_to_string(duration: Duration, expected: &'static str) {
        assert_eq!(duration.to_string(), expected.to_string());
    }
}
