use appinsights::{TelemetryClient, TelemetryConfig};
use chrono::{DateTime, Utc};
use http::{Response, StatusCode};
use serde_json::json;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn it_sends_one_telemetry_item() {
    //    let server = server::respose(compose(200)).create();
    //    server.

    //    let _e1 = mockito::mock("POST", "track")
    //        .with_body_from_fn(|w| w.write_all("ok"))
    //        .with_status()
    //        .create();
    //
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    let server = server().status(StatusCode::OK).create();
    dbg!(server.url());

    let config = TelemetryConfig::builder()
        .i_key("key")
        .endpoint(server.url())
        .interval(Duration::from_millis(500))
        .build();
    let client = TelemetryClient::from_config(config);

    client.track_event("--event--".into());
    thread::sleep(Duration::from_secs(3)); // todo use waitgroup instead
    let requests = server.requests.lock().expect("lock read");
    assert_eq!(requests.len(), 1)
}

struct TestServer {
    url: String,
    requests: Arc<Mutex<Vec<String>>>,
    responses: Arc<Vec<Response<String>>>,
}

impl TestServer {
    fn url(&self) -> &str {
        &self.url
    }
}

struct Builder {
    requests: Vec<String>,
    responses: Vec<Response<String>>,
}

fn server() -> Builder {
    Builder {
        requests: Vec::new(),
        responses: Vec::new(),
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

        let response = builder.body(body).unwrap();

        self.responses.push(response);

        self
    }

    fn status(mut self, status: StatusCode) -> Self {
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

        let requests = Arc::new(Mutex::new(Vec::new()));
        let requests_copy = Arc::clone(&requests);

        let responses = Arc::new(self.responses);
        let responses_copy = Arc::clone(&responses);

        thread::spawn(move || {
            let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

            let url = match listener.local_addr() {
                Ok(addr) => Some(format!("http://{}/track", addr)),
                Err(_) => None,
            };

            tx.send(url).unwrap();

            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
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
                            dbg!(&chunk);
                            body.push_str(&chunk);
                        }

                        let mut requests = requests_copy.lock().expect("lock");
                        requests.push(body);

                        if let Some(response) = responses_copy.first() {
                            let line = format!("HTTP/1.1 {}\r\n\r\n", response.status());
                            dbg!(&line);
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
            requests,
            responses,
        }
    }
}
