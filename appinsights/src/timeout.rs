pub use imp::*;

#[cfg(not(test))]
mod imp {
    use std::time::Duration;

    use tokio::time::{self, Instant};

    /// Creates a receiver that reliably delivers only one message when given interval expires.
    pub async fn after(duration: Duration) -> Instant {
        let timeout = Instant::now() + duration;
        time::sleep_until(timeout).await;
        timeout
    }
}

#[cfg(test)]
mod imp {
    use std::{sync::RwLock, time::Duration};

    use lazy_static::lazy_static;
    use tokio::{
        sync::mpsc::{self, Receiver, Sender},
        time::Instant,
    };

    lazy_static! {
        static ref CHANNEL: RwLock<Option<(Sender<Instant>, Receiver<Instant>)>> = RwLock::new(None);
    }

    /// Initializes a channel which emulates timeout expiration event. External code should run
    /// [`expire`](#method.expire) method in order to emulate timeout expiration.
    pub fn init() {
        let mut channel = CHANNEL.write().expect("lock");
        *channel = Some(mpsc::channel(1));
    }

    /// Creates a copy of a receiver that delivers a current time stamp in order to emulate
    /// timeout expiration for tests.
    pub async fn after(duration: Duration) -> Instant {
        // CHANNEL
        //     .read()
        //     .expect("lock")
        //     .as_ref()
        //     .map_or_else(|| crossbeam_channel::after(duration), |(_, receiver)| receiver.clone())

        if let Some((_, receiver)) = CHANNEL.write().expect("lock").as_mut() {
            receiver.recv().await.expect("instant")
        } else {
            let timeout = Instant::now() + duration;
            tokio::time::sleep_until(timeout).await;
            timeout
        }
    }

    /// Emulates timeout expiration event.
    /// It sends a current time stamp to receiver in order to trigger an action if a channel was
    /// initialized in advance. Does nothing otherwise.
    pub async fn expire() {
        if let Some((sender, _)) = CHANNEL.read().expect("lock").as_ref() {
            sender.send(Instant::now()).await.unwrap();
        }
    }

    /// Resets a channel that emulates timeout expiration event with default
    /// crossbeam_channel::bounded() instead.
    pub fn reset() {
        let mut channel = CHANNEL.write().expect("lock");
        *channel = None;
    }
}
