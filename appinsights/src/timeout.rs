pub use imp::*;

#[cfg(not(test))]
mod imp {
    use std::time::Duration;

    use tokio::time::{self, Instant};

    /// Creates a receiver that reliably delivers only one message when given interval expires.
    pub async fn sleep(duration: Duration) {
        let timeout = Instant::now() + duration;
        time::sleep_until(timeout).await;
    }
}

#[cfg(test)]
mod imp {
    use std::{sync::Arc, time::Duration};

    use lazy_static::lazy_static;
    use parking_lot::Mutex;
    use tokio::{sync::Notify, time::Instant};

    lazy_static! {
        static ref CHANNEL: Mutex<Option<Arc<Notify>>> = Mutex::new(None);
    }

    /// Initializes a channel which emulates timeout expiration event. External code should run
    /// [`expire`](#method.expire) method in order to emulate timeout expiration.
    pub fn init() {
        let mut channel = CHANNEL.lock();
        *channel = Some(Arc::new(Notify::new()));
    }

    /// Creates a copy of a receiver that delivers a current time stamp in order to emulate
    /// timeout expiration for tests.
    pub async fn sleep(duration: Duration) {
        let maybe_notify = CHANNEL.lock().clone();

        if let Some(notify) = maybe_notify {
            notify.notified().await;
        } else {
            let timeout = Instant::now() + duration;
            tokio::time::sleep_until(timeout).await;
        }
    }

    /// Emulates timeout expiration event.
    /// It sends a current time stamp to receiver in order to trigger an action if a channel was
    /// initialized in advance. Does nothing otherwise.
    pub fn expire() {
        if let Some(notify) = CHANNEL.lock().clone() {
            log::error!("notify_one");
            notify.notify_one();
        }
    }

    /// Resets a channel that emulates timeout expiration event with default
    /// timer base timeout expiration instead.
    pub fn reset() {
        let mut channel = CHANNEL.lock();
        *channel = None;
    }
}
