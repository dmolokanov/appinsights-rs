use std::{mem, sync::Arc, time::Duration};

use crossbeam_queue::SegQueue;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::{Future, Stream, StreamExt};
use log::{debug, error, trace};
use sm::{sm, Event};

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
    items: Arc<SegQueue<Envelope>>,
    command_receiver: UnboundedReceiver<Command>,
    interval: Duration,
}

impl Worker {
    pub fn new(
        transmitter: Transmitter,
        items: Arc<SegQueue<Envelope>>,
        command_receiver: UnboundedReceiver<Command>,
        interval: Duration,
    ) -> Self {
        Self {
            transmitter,
            items,
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

        let timeout = timeout::sleep(self.interval);
        items.clear();

        loop {
            tokio::select! {
                command = self.command_receiver.next() => {
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
        &mut self,
        m: Machine<Sending, E>,
        items: &mut Vec<Envelope>,
        retry: &mut Retry,
    ) -> Variant {
        *retry = Retry::exponential();
        self.handle_sending(m, items).await
    }

    async fn handle_sending_once_and_terminate<E: Event>(
        &mut self,
        m: Machine<Sending, E>,
        items: &mut Vec<Envelope>,
        retry: &mut Retry,
    ) -> Variant {
        *retry = Retry::once();
        let cloned = m.clone(); // clone here
        self.handle_sending(m, items).await;
        cloned.transition(TerminateRequested).as_enum()
    }

    async fn handle_sending<E: Event>(&mut self, m: Machine<Sending, E>, items: &mut Vec<Envelope>) -> Variant {
        // read pending items from a channel
        while let Some(item) = self.items.pop() {
            items.push(item);
        }

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
            match self.transmitter.send(mem::take(items)).await {
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
            let timeout = timeout::sleep(timeout);

            // wait for either retry timeout expired or stop command received
            tokio::select! {
                command = skip_flush(&mut self.command_receiver) => {
                    match command {
                        Some(Command::Terminate) => m.transition(TerminateRequested).as_enum(),
                        Some(Command::Close) => m.transition(CloseRequested).as_enum(),
                        Some(Command::Flush) => panic!("whoops Flush is not supported here"),
                        None => {
                            error!("commands channel closed");
                            m.transition(TerminateRequested).as_enum()
                        }
                    }
                },
                _ = timeout => {
                    debug!("Retry timeout expired");
                    m.transition(TimeoutExpired).as_enum()
                },
            }
        } else {
            debug!("All retries exhausted by {:?}", m.state());
            m.transition(RetryExhausted).as_enum()
        }
    }
}

fn skip_flush<St>(stream: &mut St) -> SkipFlush<'_, St> {
    SkipFlush { stream }
}

struct SkipFlush<'a, St: ?Sized> {
    stream: &'a mut St,
}

impl<St: ?Sized + Stream<Item = Command> + Unpin> Future for SkipFlush<'_, St> {
    type Output = Option<St::Item>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.stream.poll_next_unpin(cx) {
            std::task::Poll::Ready(Some(Command::Flush)) => std::task::Poll::Pending,
            std::task::Poll::Ready(command) => std::task::Poll::Ready(command),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
