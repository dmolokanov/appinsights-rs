#[cfg(not(test))]
pub mod imp {
    use chrono::{DateTime, Utc};

    /// Returns a DataTime which corresponds a current date.
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(test)]
pub mod imp {
    use std::cell::RefCell;

    use chrono::{DateTime, Utc};

    thread_local!(static NOW: RefCell<Option<DateTime<Utc>>> = RefCell::new(None));

    /// Returns a DataTime which corresponds a current date or the value user set in advance.
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
