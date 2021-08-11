use std::{mem, time::Duration};

use log::{debug, error, trace};
use sm::{sm, Event};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{
    channel::command::Command,
    channel::retry::Retry,
    channel::state::worker::{Variant::*, *},
    contracts::Envelope,
    timeout,
    transmitter::{Response, Transmitter},
};

sm! {
    worker {
        InitialStates { Receiving }

        TimeoutExpired {
            Receiving => Sending,
            Waiting => Sending
        }

        FlushRequested {
            Receiving => Sending
        }

        CloseRequested {
            Receiving => Sending,
            Waiting => Stopped
        }

        ItemsSentAndContinue {
            Sending => Receiving
        }

        ItemsSentAndStop {
            Sending => Stopped
        }

        RetryRequested {
            Sending => Waiting
        }

        RetryExhausted {
            Waiting => Receiving
        }

        TerminateRequested {
            Receiving => Stopped,
            Sending => Stopped,
            Waiting => Stopped
        }
    }
}

pub struct Worker {
    transmitter: Transmitter,
    event_receiver: UnboundedReceiver<Envelope>,
    command_receiver: UnboundedReceiver<Command>,
    interval: Duration,
}

impl Worker {
    pub fn new(
        transmitter: Transmitter,
        event_receiver: UnboundedReceiver<Envelope>,
        command_receiver: UnboundedReceiver<Command>,
        interval: Duration,
    ) -> Self {
        Self {
            transmitter,
            event_receiver,
            command_receiver,
            interval,
        }
    }

    pub async fn run(mut self) {
        let mut state = Machine::new(Receiving).as_enum();

        let mut items: Vec<Envelope> = Default::default();
        let mut retry = Retry::default();

        loop {
            state = match state {
                InitialReceiving(m) => self.handle_receiving(m, &mut items).await,
                ReceivingByItemsSentAndContinue(m) => self.handle_receiving(m, &mut items).await,
                ReceivingByRetryExhausted(m) => self.handle_receiving(m, &mut items).await,
                SendingByTimeoutExpired(m) => self.handle_sending_with_retry(m, &mut items, &mut retry).await,
                SendingByFlushRequested(m) => self.handle_sending_with_retry(m, &mut items, &mut retry).await,
                SendingByCloseRequested(m) => self.handle_sending_once_and_terminate(m, &mut items, &mut retry).await,
                WaitingByRetryRequested(m) => self.handle_waiting(m, &mut retry).await,
                StoppedByItemsSentAndStop(_) => break,
                StoppedByCloseRequested(_) => break,
                StoppedByTerminateRequested(_) => break,
            }
        }
    }

    async fn handle_receiving<E: Event>(&mut self, m: Machine<Receiving, E>, items: &mut Vec<Envelope>) -> Variant {
        debug!("Receiving messages triggered by {:?}", m.trigger());

        let timeout = timeout::after(self.interval);
        items.clear();

        loop {
            tokio::select! {
                command = self.command_receiver.recv() => {
                    match command {
                        Some(command) => {
                            trace!("Command received: {}", command);
                            match command {
                                Command::Flush => return m.transition(FlushRequested).as_enum(),
                                Command::Terminate => return m.transition(TerminateRequested).as_enum(),
                                Command::Close => return m.transition(CloseRequested).as_enum(),
                            }
                        },
                        None => {
                            error!("commands channel closed");
                            return m.transition(TerminateRequested).as_enum()
                        },
                    }
                },
                _ = timeout => {
                    debug!("Timeout expired");
                    return m.transition(TimeoutExpired).as_enum()
                },
            }
        }
    }

    async fn handle_sending_with_retry<E: Event>(
        &self,
        m: Machine<Sending, E>,
        items: &mut Vec<Envelope>,
        retry: &mut Retry,
    ) -> Variant {
        *retry = Retry::exponential();
        self.handle_sending(m, items)
    }

    async fn handle_sending_once_and_terminate<E: Event>(
        &self,
        m: Machine<Sending, E>,
        items: &mut Vec<Envelope>,
        retry: &mut Retry,
    ) -> Variant {
        *retry = Retry::once();
        let cloned = m.clone(); // clone here
        self.handle_sending(m, items);
        cloned.transition(TerminateRequested).as_enum()
    }

    async fn handle_sending<E: Event>(&mut self, m: Machine<Sending, E>, items: &mut Vec<Envelope>) -> Variant {
        // read items from a channel
        let pending_items = self.event_receiver.items.extend(pending_items);

        debug!(
            "Sending {} telemetry items triggered by {:?}",
            items.len(),
            m.trigger().unwrap()
        );

        // submit items to the server if any
        if items.is_empty() {
            debug!("Nothing to send. Continue to wait");
            m.transition(ItemsSentAndContinue).as_enum()
        } else {
            // attempt to send items
            match self.transmitter.send(mem::take(items)) {
                Ok(Response::Success) => m.transition(ItemsSentAndContinue).as_enum(),
                Ok(Response::Retry(retry_items)) => {
                    *items = retry_items;
                    m.transition(RetryRequested).as_enum()
                }
                Ok(Response::Throttled(_retry_after, retry_items)) => {
                    *items = retry_items;
                    // TODO implement throttling instead
                    m.transition(RetryRequested).as_enum()
                }
                Ok(Response::NoRetry) => m.transition(ItemsSentAndContinue).as_enum(),
                Err(err) => {
                    debug!("Error occurred during sending telemetry items: {}", err);
                    m.transition(RetryRequested).as_enum()
                }
            }
        }
    }

    async fn handle_waiting<E: Event>(&mut self, m: Machine<Waiting, E>, retry: &mut Retry) -> Variant {
        if let Some(timeout) = retry.next() {
            debug!(
                "Waiting for retry timeout {:?} or stop command triggered by {:?}",
                timeout,
                m.state()
            );
            // sleep until next sending attempt
            let timeout = timeout::after(timeout);
            tokio::pin!(timeout);

            // wait for either retry timeout expired or stop command received
            loop {
                // let command_recv = ;
                // tokio::pin!(command_recv);

                tokio::select! {
                    command = self.command_receiver.recv() => {
                        match command {
                            Some(command) => match command {
                                Command::Flush => continue,
                                Command::Terminate => return m.transition(TerminateRequested).as_enum(),
                                Command::Close => return m.transition(CloseRequested).as_enum(),
                            },
                            None => {
                                error!("commands channel closed");
                                return m.transition(TerminateRequested).as_enum()
                            }
                        }
                    },
                    _ = timeout => {
                        debug!("Retry timeout expired");
                        return m.transition(TimeoutExpired).as_enum()
                    },
                }
            }
        } else {
            debug!("All retries exhausted by {:?}", m.state());
            m.transition(RetryExhausted).as_enum()
        }
    }
}
