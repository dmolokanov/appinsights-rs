use std::{
    env,
    sync::{Arc, RwLock},
    time::Duration,
};

use appinsights::{telemetry::SeverityLevel, TelemetryClient};
use hyper::{Method, Uri};

#[tokio::test]
async fn it_tracks_all_telemetry_items() {
    let entries = Arc::new(RwLock::new(Vec::new()));
    logger::builder(entries.clone()).output(true).init();

    let i_key = env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");
    let ai = TelemetryClient::new(i_key);

    ai.track_event("event happened");
    ai.track_trace("Unable to connect to a gateway", SeverityLevel::Warning);
    ai.track_metric("gateway_latency_ms", 113.0);
    ai.track_request(
        Method::GET,
        "https://api.github.com/dmolokanov/appinsights-rs"
            .parse::<Uri>()
            .unwrap(),
        Duration::from_millis(100),
        "200".to_string(),
    );
    ai.track_remote_dependency(
        "GET https://api.github.com/dmolokanov/appinsights-rs",
        "HTTP",
        "api.github.com",
        true,
    );
    ai.track_availability(
        "GET https://api.github.com/dmolokanov/appinsights-rs",
        Duration::from_secs(2),
        true,
    );

    ai.close_channel().await;

    logger::wait_until(&entries, "Successfully sent 6 items", Duration::from_secs(10)).await;
}

pub mod logger {
    use std::{
        sync::{Arc, RwLock},
        time::Duration,
    };

    use chrono::Utc;
    use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

    pub async fn wait_until(entries: &Arc<RwLock<Vec<String>>>, msg: &str, panic_after: Duration) {
        let panic_after = Utc::now() + chrono::Duration::from_std(panic_after).unwrap();
        loop {
            let entries = entries.read().unwrap();
            if entries.iter().any(|entry| entry.contains(msg)) {
                break;
            }
            drop(entries);

            if Utc::now() > panic_after {
                panic!("Test took too long to finish");
            }
            tokio::time::sleep(Duration::from_millis(100)).await
        }
    }

    pub fn init(entries: Arc<RwLock<Vec<String>>>) {
        builder(entries).init()
    }

    pub fn builder(entries: Arc<RwLock<Vec<String>>>) -> Builder {
        Builder::new(entries)
    }

    pub struct Builder {
        entries: Arc<RwLock<Vec<String>>>,
        level: Level,
        output: bool,
    }

    impl Builder {
        pub fn new(entries: Arc<RwLock<Vec<String>>>) -> Self {
            Self {
                entries,
                level: Level::Debug,
                output: false,
            }
        }

        pub fn level(&mut self, level: Level) -> &mut Self {
            self.level = level;
            self
        }

        pub fn output(&mut self, output: bool) -> &mut Self {
            self.output = output;
            self
        }

        pub fn init(&mut self) {
            self.try_init().expect("Builder::init failed")
        }

        pub fn try_init(&mut self) -> Result<(), SetLoggerError> {
            let logger = MemoryLogger {
                entries: self.entries.clone(),
                level: self.level,
                output: self.output,
            };

            log::set_boxed_logger(Box::new(logger))?;
            log::set_max_level(LevelFilter::Trace);
            Ok(())
        }
    }

    struct MemoryLogger {
        entries: Arc<RwLock<Vec<String>>>,
        level: Level,
        output: bool,
    }

    impl Log for MemoryLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= self.level
        }

        fn log(&self, record: &Record) {
            if !self.enabled(record.metadata()) {
                return;
            }

            let entry = format!(
                "{} {:<5} [{}] {}",
                chrono::Utc::now().to_rfc3339(),
                record.level(),
                if !record.target().is_empty() {
                    record.target()
                } else {
                    record.module_path().unwrap_or_default()
                },
                record.args()
            );

            if self.output {
                println!("{}", entry);
            }

            let mut entries = self.entries.write().unwrap();
            entries.push(entry);
        }

        fn flush(&self) {}
    }
}
