use std::{
    env,
    sync::{Arc, RwLock},
    time::Duration,
};

use appinsights::{telemetry::SeverityLevel, TelemetryClient};
use hyper::{Method, Uri};

#[test]
fn it_tracks_all_telemetry_items() {
    let entries = Arc::new(RwLock::new(Vec::new()));
    logger::init(entries.clone()).unwrap();

    let i_key = env::var("APPINSIGHTS_INSTRUMENTATIONKEY").expect("Set APPINSIGHTS_INSTRUMENTATIONKEY first");
    let ai = TelemetryClient::new(i_key);

    ai.track_event("event happened".into());
    ai.track_trace("Unable to connect to a gateway".to_string(), SeverityLevel::Warning);
    ai.track_metric("gateway_latency_ms".to_string(), 113.0);
    ai.track_request(
        Method::GET,
        "https://api.github.com/dmolokanov/appinsights-rs"
            .parse::<Uri>()
            .unwrap(),
        Duration::from_millis(100),
        "200".to_string(),
    );
    ai.track_remote_dependency(
        "GET https://api.github.com/dmolokanov/appinsights-rs".to_string(),
        "HTTP".to_string(),
        "api.github.com".to_string(),
        true,
    );
    ai.track_availability(
        "GET https://api.github.com/dmolokanov/appinsights-rs".to_string(),
        Duration::from_secs(2),
        true,
    );

    // TODO make something more reliable
    std::thread::sleep(Duration::from_millis(300));
    ai.close_channel();

    logger::wait_until(&entries, "Successfully sent 6 items", Duration::from_secs(10));
}

pub mod logger {
    use std::{
        sync::{Arc, RwLock},
        time::Duration,
    };

    use chrono::Utc;
    use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

    pub fn wait_until(entries: &Arc<RwLock<Vec<String>>>, msg: &str, panic_after: Duration) {
        let panic_after = Utc::now() + chrono::Duration::from_std(panic_after).unwrap();
        loop {
            let entries = entries.read().unwrap();
            if entries.iter().any(|entry| entry.contains(msg)) {
                break;
            }

            if Utc::now() > panic_after {
                panic!("Test took too long to finish");
            }
            std::thread::sleep(Duration::from_millis(100))
        }
    }

    struct MemoryLogger {
        entries: Arc<RwLock<Vec<String>>>,
        level: Level,
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
            println!("{}", entry);

            let mut entries = self.entries.write().unwrap();
            entries.push(entry);
        }

        fn flush(&self) {}
    }

    pub fn init(entries: Arc<RwLock<Vec<String>>>) -> Result<(), SetLoggerError> {
        init_with_level(entries, Level::Debug)
    }

    pub fn init_with_level(entries: Arc<RwLock<Vec<String>>>, level: Level) -> Result<(), SetLoggerError> {
        let logger = MemoryLogger { entries, level };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
}
