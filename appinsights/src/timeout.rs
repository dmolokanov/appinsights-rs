pub use imp::*;

#[cfg(not(test))]
mod imp {
    use std::time::{Duration, Instant};

    use crossbeam_channel::Receiver;

    /// Creates a receiver that reliably delivers only one message when given interval expires.
    pub fn after(duration: Duration) -> Receiver<Instant> {
        crossbeam_channel::after(duration)
    }
}

#[cfg(test)]
mod imp {
    use std::sync::RwLock;
    use std::time::{Duration, Instant};

    use crossbeam_channel::{Receiver, Sender};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CHANNEL: RwLock<Option<(Sender<Instant>, Receiver<Instant>)>> = RwLock::new(None);
    }

    /// Initializes a channel which emulates timeout expiration event. External code should run
    /// [`expire`](#method.expire) method in order to emulate timeout expiration.
    pub fn init() {
        let mut channel = CHANNEL.write().expect("lock");
        *channel = Some(crossbeam_channel::bounded(1));
    }

    /// Creates a copy of a receiver that delivers a current time stamp in order to emulate
    /// timeout expiration for tests.
    pub fn after(duration: Duration) -> Receiver<Instant> {
        CHANNEL
            .read()
            .expect("lock")
            .as_ref()
            .map_or_else(|| crossbeam_channel::after(duration), |(_, receiver)| receiver.clone())
    }

    /// Emulates timeout expiration event.
    /// It sends a current time stamp to receiver in order to trigger an action if a channel was
    /// initialized in advance. Does't nothing otherwise.
    pub fn expire() {
        if let Some((sender, _)) = CHANNEL.read().expect("lock").as_ref() {
            sender.send(Instant::now()).unwrap();
        }
    }

    /// Resets a channel that emulates timeout expiration event with default
    /// crossbeam_channel::bounded() instead.
    pub fn reset() {
        let mut channel = CHANNEL.write().expect("lock");
        *channel = None;
    }
}
