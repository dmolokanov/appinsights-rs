use std::time::Duration;

use crossbeam_channel::{after, select, Receiver};
use log::{debug, error, info};
use sm::{sm, Event};

use crate::contracts::Envelope;
use crate::transmitter::Transmitter;

use crate::channel::command::Command;
use crate::channel::retry::Retry;
use crate::channel::state::worker::{Variant::*, *};

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
    event_receiver: Receiver<Envelope>,
    command_receiver: Receiver<Command>,
    interval: Duration,
}

impl Worker {
    pub fn new(
        transmitter: Transmitter,
        event_receiver: Receiver<Envelope>,
        command_receiver: Receiver<Command>,
        interval: Duration,
    ) -> Self {
        Self {
            transmitter,
            event_receiver,
            command_receiver,
            interval,
        }
    }

    pub fn run(&self) {
        let mut state = Machine::new(Receiving).as_enum();

        let mut items: Vec<Envelope> = Default::default();
        let mut retry = Retry::default();

        loop {
            state = match state {
                InitialReceiving(m) => self.handle_receiving(m, &mut items),
                ReceivingByItemsSentAndContinue(m) => self.handle_receiving(m, &mut items),
                ReceivingByRetryExhausted(m) => self.handle_receiving(m, &mut items),
                SendingByTimeoutExpired(m) => self.handle_sending_with_retry(m, &mut items, &mut retry),
                SendingByFlushRequested(m) => self.handle_sending_with_retry(m, &mut items, &mut retry),
                SendingByCloseRequested(m) => self.handle_sending_once_and_terminate(m, &mut items, &mut retry),
                WaitingByRetryRequested(m) => self.handle_waiting(m, &mut retry),
                StoppedByItemsSentAndStop(_) => break,
                StoppedByCloseRequested(_) => break,
                StoppedByTerminateRequested(_) => break,
            }
        }
    }

    fn handle_receiving<E: Event>(&self, m: Machine<Receiving, E>, items: &mut Vec<Envelope>) -> Variant {
        debug!("Receiving messages triggered by {:?}", m.trigger());

        let timeout = after(self.interval);
        items.clear();

        loop {
            select! {
                recv(self.event_receiver) -> event => {
                    match event {
                        Ok(envelope) => {
                            items.push(envelope);
                            continue
                        },
                        Err(err) => {
                            error!("event channel closed: {}", err);
                            return m.transition(TerminateRequested).as_enum()
                        }
                    }
                }
                recv(self.command_receiver) -> command => {
                    match command {
                        Ok(command) => match command {
                            Command::Flush => return m.transition(FlushRequested).as_enum(),
                            Command::Terminate => return m.transition(TerminateRequested).as_enum(),
                            Command::Close => return m.transition(CloseRequested).as_enum(),
                        },
                        Err(err) => {
                            error!("commands channel closed: {}", err);
                            return m.transition(TerminateRequested).as_enum()
                        },
                    }
                },
                recv(timeout) -> _ => {
                    debug!("Timeout expired");
                    return m.transition(TimeoutExpired).as_enum()
                },
            }
        }
    }

    fn handle_sending_with_retry<E: Event>(
        &self,
        m: Machine<Sending, E>,
        items: &mut Vec<Envelope>,
        retry: &mut Retry,
    ) -> Variant {
        *retry = Retry::exponential();
        self.handle_sending(m, items)
    }

    fn handle_sending_once_and_terminate<E: Event>(
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

    fn handle_sending<E: Event>(&self, m: Machine<Sending, E>, items: &mut Vec<Envelope>) -> Variant {
        debug!(
            "Sending {} telemetry items triggered by {:?}",
            items.len(),
            m.trigger().unwrap()
        );

        // attempt to send items
        match self.transmitter.transmit(&items) {
            Ok(transmission) => {
                info!(
                    "Successfully sent {}/{} telemetry items",
                    transmission.accepted(),
                    transmission.received()
                );

                if transmission.is_success() {
                    return m.transition(ItemsSentAndContinue).as_enum();
                }

                // make an attempt to re-send only if there are any items in the list that can be re-sent
                if transmission.can_retry() {
                    *items = items
                        .drain(..)
                        .into_iter()
                        .enumerate()
                        .filter_map(|(i, envelope)| {
                            if transmission.can_retry_item(i) {
                                Some(envelope)
                            } else {
                                None
                            }
                        })
                        .collect();
                    if items.is_empty() {
                        return m.transition(ItemsSentAndContinue).as_enum();
                    }
                }
            }
            Err(err) => info!("Error occurred during sending telemetry items: {}", err),
        }

        return m.transition(RetryRequested).as_enum();
    }

    fn handle_waiting<E: Event>(&self, m: Machine<Waiting, E>, retry: &mut Retry) -> Variant {
        debug!(
            "Waiting for timeout {:?} or stop command triggered by {:?}",
            retry,
            m.state()
        );
        if let Some(timeout) = retry.next() {
            // sleep until next sending attempt
            let timeout = after(timeout);

            // wait for either timeout expired or stop command received
            loop {
                select! {
                    recv(self.command_receiver) -> command => {
                        match command {
                            Ok(command) => match command {
                                Command::Flush => continue,
                                Command::Terminate => return m.transition(TerminateRequested).as_enum(),
                                Command::Close => return m.transition(CloseRequested).as_enum(),
                            },
                            Err(err) => {
                                error!("commands channel closed: {}", err);
                                return m.transition(TerminateRequested).as_enum()
                            }
                        }
                    },
                    recv(timeout) -> _ => {
                        debug!("Timeout expired");
                        return m.transition(TimeoutExpired).as_enum() // todo
                    },
                }
            }
        } else {
            return m.transition(RetryExhausted).as_enum();
        }
    }
}
