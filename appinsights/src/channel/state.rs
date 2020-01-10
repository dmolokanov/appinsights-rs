use std::time::Duration;

use crossbeam_channel::{select, Receiver};
use log::{debug, error, trace};
use sm::{sm, Event};

use crate::contracts::Envelope;
use crate::timeout;
use crate::transmitter::{Response, Transmitter};

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

        let timeout = timeout::after(self.interval);
        items.clear();

        loop {
            select! {
                recv(self.event_receiver) -> event => {
                    match event {
                        Ok(envelope) => {
                            trace!("Telemetry item received: {}", envelope.name);
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
                        Ok(command) => {
                            trace!("Command received: {}", command);
                            match command {
                                Command::Flush => return m.transition(FlushRequested).as_enum(),
                                Command::Terminate => return m.transition(TerminateRequested).as_enum(),
                                Command::Close => return m.transition(CloseRequested).as_enum(),
                            }
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

        if items.is_empty() {
            debug!("Nothing to send. Continue to wait");
            m.transition(ItemsSentAndContinue).as_enum()
        } else {
            // attempt to send items
            match self.transmitter.send(items) {
                Ok(Response::Success) => {
                    items.clear();
                    m.transition(ItemsSentAndContinue).as_enum()
                }
                Ok(Response::Retry(retry_items)) => {
                    *items = retry_items;
                    m.transition(RetryRequested).as_enum()
                }
                Ok(Response::Throttled(_retry_after, retry_items)) => {
                    *items = retry_items;
                    // TODO implement throttling instead
                    m.transition(RetryRequested).as_enum()
                }
                Ok(Response::NoRetry) => {
                    items.clear();
                    m.transition(ItemsSentAndContinue).as_enum()
                }
                Err(err) => {
                    debug!("Error occurred during sending telemetry items: {}", err);
                    m.transition(RetryRequested).as_enum()
                }
            }
        }
    }

    fn handle_waiting<E: Event>(&self, m: Machine<Waiting, E>, retry: &mut Retry) -> Variant {
        if let Some(timeout) = retry.next() {
            debug!(
                "Waiting for retry timeout {:?} or stop command triggered by {:?}",
                timeout,
                m.state()
            );
            // sleep until next sending attempt
            let timeout = timeout::after(timeout);

            // wait for either retry timeout expired or stop command received
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
