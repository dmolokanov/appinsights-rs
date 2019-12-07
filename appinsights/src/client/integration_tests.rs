use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{DateTime, Utc};
use crossbeam_channel::{unbounded, Receiver, RecvTimeoutError};
use futures::future;
use futures::sync::oneshot;
use hyper::rt::{Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};
use lazy_static::lazy_static;
use matches::assert_matches;
use serde_json::json;

use crate::channel::InMemoryChannel;
use crate::{timeout, TelemetryClient, TelemetryConfig};

lazy_static! {
    /// A global lock since most tests need to run in serial.
    static ref SERIAL_TEST_MUTEX: Mutex<()> = Mutex::new(());
}

/// Macro to crate a serial test, that locks the `SERIAL_TEST_MUTEX` while testing.
macro_rules! serial_test {
    (fn $name: ident() $body: block) => {
        #[test]
        fn $name() {
            let guard = SERIAL_TEST_MUTEX.lock().unwrap();
            // Catch any panics to not poison the lock.
            if let Err(err) = std::panic::catch_unwind(|| $body) {
                drop(guard);

                std::panic::resume_unwind(err);
            }
        }
    };
}

macro_rules! manual_timeout_test {
    (fn $name: ident() $body: block) => {
        #[test]
        fn $name() {
            let guard = SERIAL_TEST_MUTEX.lock().unwrap();
            timeout::init();

            // Catch any panics to not poison the lock.
            if let Err(err) = std::panic::catch_unwind(|| $body) {
                drop(guard);

                std::panic::resume_unwind(err);
            }

            timeout::reset();
        }
    };
}

manual_timeout_test! {
    fn it_sends_one_telemetry_item() {
        let server = server().status(StatusCode::OK).create();

        let client = create_client(server.url());
        client.track_event("--event--".into());

        timeout::expire();

        // expect one requests available so far
        let receiver = server.requests();
        assert_matches!(receiver.recv_timeout(Duration::from_millis(500)), Ok(_));
    }
}

manual_timeout_test! {
    fn it_does_not_resend_submitted_telemetry_items() {
        let server = server().status(StatusCode::OK).create();

        let client = create_client(server.url());
        client.track_event("--event--".into());

        // verify 1 items is sent after first interval expired
        let receiver = server.requests();
        timeout::expire();
        assert_matches!(receiver.recv_timeout(Duration::from_millis(500)), Ok(_));

        // verify no items is sent after next interval expired
        timeout::expire();
        assert_matches!(
            receiver.recv_timeout(Duration::from_millis(500)),
            Err(RecvTimeoutError::Timeout)
        );
    }
}

manual_timeout_test! {
    fn it_sends_telemetry_items_in_2_batches() {
        let server = server().status(StatusCode::OK).status(StatusCode::OK).create();

        let client = create_client(server.url());

        // send 10 items and then interval expired
        for i in 0..10 {
            client.track_event(format!("--event {}--", i));
        }
        timeout::expire();

        // send next 5 items and then interval expired
        for i in 10..15 {
            client.track_event(format!("--event {}--", i));
        }
        // TODO delete this hack
        // this thread::sleep is required only to await while all items sent in previous step be
        // processed buy internal worker. Now it contains multiple channels that worker loop reads
        // events from one by one sometimes it picks expiration command instead of items sent
        // before.
        std::thread::sleep(Duration::from_millis(300));
        timeout::expire();

        // verify that 2 requests has been send
        let requests = server.wait_for_requests(2);
        assert_eq!(requests.len(), 2);

        // verify that all requests are available
        let content = requests.into_iter().fold(String::new(), |mut content, body| {
            content.push_str(&body);
            content
        });
        let items_count = (0..15)
            .filter_map(|i| {
                if content.contains(&format!("--event {}--", i)) {
                    Some(i)
                } else {
                    None
                }
            })
            .count();
        assert_eq!(items_count, 15);
    }
}

manual_timeout_test! {
    fn it_flushes_all_pending_telemetry_items() {
        let server = server().status(StatusCode::OK).status(StatusCode::OK).create();

        let client = create_client(server.url());

        // send 15 items and then interval expired
        for i in 0..15 {
            client.track_event(format!("--event {}--", i));
        }

        // TODO delete this hack
        // this thread::sleep is required only to await while all items sent in previous step be
        // processed buy internal worker. Now it contains multiple channels that worker loop reads
        // events from one by one sometimes it picks expiration command instead of items sent
        // before.
        std::thread::sleep(Duration::from_millis(300));

        // force client to send all items to the server
        client.flush_channel();

        // NOTE no timeout expired
        // assert that 1 request has been sent
        let requests = server.wait_for_requests(1);
        assert_eq!(requests.len(), 1);

        // verify request contains all items we submitted to the client
        let content = requests.into_iter().fold(String::new(), |mut content, body| {
            content.push_str(&body);
            content
        });
        let items_count = (0..15)
            .filter_map(|i| {
                if content.contains(&format!("--event {}--", i)) {
                    Some(i)
                } else {
                    None
                }
            })
            .count();
        assert_eq!(items_count, 15);
    }
}

manual_timeout_test! {
    fn it_does_not_send_any_pending_telemetry_items_when_drop_client() {
        let server = server().status(StatusCode::OK).status(StatusCode::OK).create();

        let client = create_client(server.url());

        // send 15 items and then interval expired
        for i in 0..15 {
            client.track_event(format!("--event {}--", i));
        }

        // TODO delete this hack
        // this thread::sleep is required only to await while all items sent in previous step be
        // processed buy internal worker. Now it contains multiple channels that worker loop reads
        // events from one by one sometimes it picks expiration command instead of items sent
        // before.
        std::thread::sleep(Duration::from_millis(300));

        // drop client
        drop(client);

        // verify that nothing has been sent to the server
        let receiver = server.requests();
        assert_matches!(
                receiver.recv_timeout(Duration::from_millis(500)),
                Err(RecvTimeoutError::Timeout)
            );

    }
}

fn create_client(endpoint: &str) -> TelemetryClient<InMemoryChannel> {
    let config = TelemetryConfig::builder()
        .i_key("instrumentation key")
        .endpoint(endpoint)
        .interval(Duration::from_millis(300))
        .build();

    TelemetryClient::from_config(config)
}

struct Builder {
    responses: Vec<Response<String>>,
}

fn server() -> Builder {
    Builder { responses: Vec::new() }
}

struct HyperTestServer {
    url: String,
    requests: Receiver<String>,
    shutdown: Option<futures::sync::oneshot::Sender<()>>,
}

impl HyperTestServer {
    fn url(&self) -> &str {
        &self.url
    }

    fn requests(&self) -> Receiver<String> {
        self.requests.clone()
    }

    fn wait_for_requests(&self, count: usize) -> Vec<String> {
        let mut requests = Vec::new();

        for _ in 0..count {
            match self.requests.recv_timeout(Duration::from_millis(1000)) {
                Result::Ok(request) => requests.push(request),
                Result::Err(err) => {
                    dbg!(err);
                }
            }
        }

        requests
    }
}

impl Drop for HyperTestServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            shutdown.send(()).unwrap();
        }
    }
}

impl Builder {
    fn response(mut self, status: StatusCode, body: String, retry_after: Option<DateTime<Utc>>) -> Self {
        let mut builder = Response::builder();
        builder.status(status);

        if let Some(retry_after) = retry_after {
            let retry_after = retry_after.to_rfc2822();
            builder.header("Retry-After", retry_after);
        }

        let response = builder.body(body.into()).unwrap();
        self.responses.push(response);

        self
    }

    fn status(self, status: StatusCode) -> Self {
        let body = json!({
            "itemsAccepted": 1,
            "itemsReceived": 1,
            "errors": [],
        })
        .to_string();
        self.response(status, body, None)
    }

    fn create(self) -> HyperTestServer {
        let (shutdown_sender, shutdown_receiver) = oneshot::channel::<()>();
        let (request_sender, request_receiver) = unbounded::<String>();

        let responses = Arc::new(self.responses);
        let counter = Arc::new(AtomicUsize::new(0));

        let new_service = move || {
            let request_sender = request_sender.clone();
            let counter = counter.clone();
            let responses = responses.clone();

            service_fn(move |req: Request<Body>| {
                let request_sender = request_sender.clone();
                let counter = counter.clone();
                let responses = responses.clone();

                req.into_body()
                    .fold(Vec::new(), |mut acc, chuck| {
                        acc.extend_from_slice(chuck.as_ref());
                        future::ok::<_, hyper::Error>(acc)
                    })
                    .and_then(move |body| {
                        let content = String::from_utf8(body).unwrap();
                        request_sender.send(content).unwrap();

                        let count = counter.fetch_add(1, Ordering::AcqRel);

                        if let Some(response) = responses.get(count) {
                            let res = Response::builder()
                                .status(response.status())
                                .body(Body::from(response.body().clone()))
                                .unwrap();
                            future::ok(res)
                        } else {
                            future::ok(
                                Response::builder()
                                    .status(StatusCode::NOT_FOUND)
                                    .body(Body::empty())
                                    .unwrap(),
                            )
                        }
                    })
            })
        };

        let server = Server::bind(&([0, 0, 0, 0], 0).into()).serve(new_service);
        let url = format!("http://{}", server.local_addr());
        let graceful = server
            .with_graceful_shutdown(shutdown_receiver)
            .map_err(|err| eprintln!("server error: {}", err));

        std::thread::spawn(move || {
            hyper::rt::run(graceful.map_err(|_| ()));
        });

        HyperTestServer {
            url,
            requests: request_receiver,
            shutdown: Some(shutdown_sender),
        }
    }
}
