#![allow(dead_code, unused_variables)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{DateTime, Utc};
use crossbeam_channel::{bounded, unbounded, Receiver, RecvTimeoutError};
use futures::stream::StreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use lazy_static::lazy_static;
use matches::assert_matches;
use serde_json::json;
use tokio::sync::oneshot;

use crate::channel::InMemoryChannel;
use crate::{timeout, TelemetryClient, TelemetryConfig};

lazy_static! {
    /// A global lock since most tests need to run in serial.
    static ref SERIAL_TEST_MUTEX: Mutex<()> = Mutex::new(());
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
            .filter(|i| content.contains(&format!("--event {}--", i)))
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
            .filter(|i| content.contains(&format!("--event {}--", i)))
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

manual_timeout_test! {
    fn it_tries_to_send_pending_telemetry_items_when_close_channel_requested() {
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

        // close internal channel means that client will make an attempt to send telemetry items once
        // and then tear down submission flow
        client.close_channel();

        // NOTE no timeout expired
        // verify that 1 request has been sent
        let requests = server.wait_for_requests(1);
        assert_eq!(requests.len(), 1);

        // verify request contains all items we submitted to the client
        let content = requests.into_iter().fold(String::new(), |mut content, body| {
            content.push_str(&body);
            content
        });
        let items_count = (0..15)
            .filter(|i| content.contains(&format!("--event {}--", i)))
            .count();
        assert_eq!(items_count, 15);
    }
}

manual_timeout_test! {
    fn it_retries_when_previous_submission_failed() {
        let server = server()
            .response(StatusCode::INTERNAL_SERVER_ERROR, json!({}), None)
            .response(
                StatusCode::OK,
                json!(
                {
                    "itemsAccepted": 15,
                    "itemsReceived": 15,
                    "errors": [],
                }),
                None,
            )
            .create();

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
        timeout::expire();

        // "wait" until retry logic handled
        std::thread::sleep(Duration::from_millis(300));
        timeout::expire();

        // verify there are 2 identical requests
        let requests = server.wait_for_requests(2);
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0], requests[1]);
    }
}

manual_timeout_test! {
    fn it_retries_when_partial_content() {
        let server = server()
            .response(
                StatusCode::PARTIAL_CONTENT,
                json!(
                {
                    "itemsAccepted": 12,
                    "itemsReceived": 15,
                    "errors": [
                        {
                            "index": 4,
                            "statusCode": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            "message": "Internal Server Error"
                        },
                        {
                            "index": 9,
                            "statusCode": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            "message": "Internal Server Error"
                        },
                        {
                            "index": 14,
                            "statusCode": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            "message": "Internal Server Error"
                        }
                    ],
                }),
                None,
            )
            .response(
                StatusCode::OK,
                json!(
                {
                    "itemsAccepted": 3,
                    "itemsReceived": 3,
                    "errors": [],
                }),
                None,
            )
            .create();

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
        timeout::expire();

        // "wait" until retry logic handled
        std::thread::sleep(Duration::from_millis(300));
        timeout::expire();

        // verify it sends a first request with all items
        let requests = server.wait_for_requests(1);
        assert_eq!(requests.len(), 1);

        let content = requests.into_iter().fold(String::new(), |mut content, body| {
            content.push_str(&body);
            content
        });
        let items_count = (0..15)
            .filter(|i| content.contains(&format!("--event {}--", i)))
            .count();
        assert_eq!(items_count, 15);

        // verify it re-send only errors that previously were invalid
        let requests = server.wait_for_requests(1);
        assert_eq!(requests.len(), 1);

        let content = requests.into_iter().fold(String::new(), |mut content, body| {
            content.push_str(&body);
            content
        });
        let items_count = [4, 9, 14]
            .iter()
            .filter(|i| content.contains(&format!("--event {}--", i)))
            .count();
        assert_eq!(items_count, 3);
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
    shutdown: Option<oneshot::Sender<()>>,
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
                    log::error!("{:?}", err);
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
    fn response(mut self, status: StatusCode, body: impl ToString, retry_after: Option<DateTime<Utc>>) -> Self {
        let mut builder = Response::builder().status(status);

        if let Some(retry_after) = retry_after {
            let retry_after = retry_after.to_rfc2822();
            builder = builder.header("Retry-After", retry_after);
        }

        let response = builder.body(body.to_string()).unwrap();
        self.responses.push(response);

        self
    }

    fn status(self, status: StatusCode) -> Self {
        self.response(
            status,
            json!(
            {
                "itemsAccepted": 1,
                "itemsReceived": 1,
                "errors": [],
            }),
            None,
        )
    }

    fn create(self) -> HyperTestServer {
        let (shutdown_sender, shutdown_receiver) = oneshot::channel::<()>();
        let (url_sender, url_receiver) = bounded::<String>(1);
        let (request_sender, request_receiver) = unbounded::<String>();

        let responses = Arc::new(self.responses);
        let counter = Arc::new(AtomicUsize::new(0));

        let make_service = make_service_fn(move |_| {
            let request_sender = request_sender.clone();
            let counter = counter.clone();
            let responses = responses.clone();

            async move {
                Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                    let request_sender = request_sender.clone();
                    let counter = counter.clone();
                    let responses = responses.clone();

                    async move {
                        let body = req
                            .into_body()
                            .fold(Vec::new(), |mut acc, chunk| async move {
                                if let Ok(chunk) = chunk {
                                    acc.extend_from_slice(chunk.as_ref());
                                }
                                acc
                            })
                            .await;
                        let content = String::from_utf8(body).unwrap();
                        request_sender.send(content).unwrap();

                        let count = counter.fetch_add(1, Ordering::AcqRel);

                        let response = if let Some(response) = responses.get(count) {
                            Response::builder()
                                .status(response.status())
                                .body(Body::from(response.body().clone()))
                                .unwrap()
                        } else {
                            Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::empty())
                                .unwrap()
                        };

                        Ok::<_, hyper::Error>(response)
                    }
                }))
            }
        });

        std::thread::spawn(move || {
            let mut rt = tokio::runtime::Runtime::new().expect("runtime");
            rt.block_on(async move {
                let server = Server::bind(&([0, 0, 0, 0], 0).into()).serve(make_service);

                let url = format!("http://{}", server.local_addr());
                url_sender.send(url).unwrap();

                let graceful = server.with_graceful_shutdown(async {
                    shutdown_receiver.await.ok();
                });

                if let Err(e) = graceful.await {
                    log::error!("server error: {}", e);
                }
            });
        });

        let url = url_receiver.recv().expect("url");

        HyperTestServer {
            url,
            requests: request_receiver,
            shutdown: Some(shutdown_sender),
        }
    }
}
