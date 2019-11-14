pub use time::SystemTime;

#[cfg(not(test))]
mod time {
    use chrono::{DateTime, Utc};

    /// Returns a DataTime which corresponds a current date.
    pub struct SystemTime;

    impl SystemTime {
        /// Returns a DataTime which corresponds a current date.
        pub fn now() -> DateTime<Utc> {
            Utc::now()
        }
    }
}

#[cfg(test)]
mod time {
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::RefCell;

    use chrono::{DateTime, Utc};

    thread_local!(static NOW: RefCell<Option<DateTime<Utc>>> = RefCell::new(None));

    /// Returns a DataTime which corresponds a current date. In addition user can set specific value
    /// to assert against it in unit tests.
    pub struct SystemTime;

    impl SystemTime {
        /// Returns a DataTime which corresponds a current date.
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
}
