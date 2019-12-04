use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};
use crossbeam_channel::{unbounded, Receiver};
use http::{Response, StatusCode};
use lazy_static::lazy_static;
use matches::assert_matches;
use serde_json::json;

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

serial_test! {
    fn it_sends_one_telemetry_item() {
        timeout::init();

        let server = server().status(StatusCode::OK).create();

        let config = TelemetryConfig::builder()
            .i_key("instrumentation key")
            .endpoint(server.url())
            .interval(Duration::from_millis(500))
            .build();

        let client = TelemetryClient::from_config(config);

        client.track_event("--event--".into());

        timeout::expire();

        // expect one requests available so far
        let receiver = server.requests();
        assert_matches!(receiver.recv_timeout(Duration::from_secs(1)), Ok(_));
    }
}

struct TestServer {
    url: String,
    //    requests: Arc<Mutex<Vec<String>>>,
    requests: Receiver<String>,
    running: Arc<AtomicBool>,
}

impl TestServer {
    fn url(&self) -> &str {
        &self.url
    }

    fn requests(&self) -> Receiver<String> {
        self.requests.clone()
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

struct Builder {
    responses: Vec<Response<String>>,
}

fn server() -> Builder {
    Builder { responses: Vec::new() }
}

impl Builder {
    fn response(mut self, status: StatusCode, body: String, retry_after: Option<DateTime<Utc>>) -> Self {
        let mut builder = Response::builder();
        builder.status(status);

        if let Some(retry_after) = retry_after {
            let retry_after = retry_after.to_rfc2822();
            builder.header("Retry-After", retry_after);
        }

        let response = builder.body(body).unwrap();
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

    fn create(self) -> TestServer {
        let (tx, rx) = mpsc::channel();

        let (request_sender, request_receiver) = unbounded();

        let mut responses = self.responses.into_iter();

        let running = Arc::new(AtomicBool::new(true));
        let running_copy = running.clone();

        thread::spawn(move || {
            let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

            let url = match listener.local_addr() {
                Ok(addr) => Some(format!("http://{}/track", addr)),
                Err(_) => None,
            };

            tx.send(url).unwrap();

            while running.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buffer = [0; 512];
                        let mut body = String::new();

                        //                        loop
                        {
                            let bytes = match stream.read(&mut buffer) {
                                Ok(bytes) => bytes,
                                Err(_) => 0,
                            };

                            if bytes <= 0 {
                                break;
                            }

                            let chunk = String::from_utf8_lossy(&buffer[..bytes]);
                            body.push_str(&chunk);
                        }

                        request_sender.send(body).unwrap();

                        if let Some(response) = responses.next() {
                            let line = format!("HTTP/1.1 {}\r\n\r\n", response.status());
                            stream.write_all(line.as_bytes()).unwrap();
                        } else {
                            let line = "HTTP/1.0 404 Not Found";
                            stream.write_all(line.as_bytes()).unwrap();
                        }

                        stream.flush().unwrap();
                    }
                    Err(err) => {
                        eprintln!("cannot read from stream: {}", err);
                    }
                }
            }
        });

        let url = rx.recv().ok().and_then(|url| url).unwrap();

        TestServer {
            url,
            requests: request_receiver,
            running: running_copy,
        }
    }
}
