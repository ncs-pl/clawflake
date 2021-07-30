use chrono::prelude::*;
use log::{Level, Log, Metadata, Record, SetLoggerError};
use serde_json::json;

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        let file: &str = record.file().unwrap_or("");
        let line: i32 = record.line().map_or(-1, |v| v as i32);
        let module_path: &str = record.module_path().unwrap_or("");
        let ts: String = Utc::now().to_string();

        match record.level() {
            Level::Error => {
                println!(
                    "{}",
                    json!({
                      "ts": ts,
                      "file": file,
                      "line": line,
                      "module_path": module_path,
                      "msg": record.args().to_string(),
                      "level": "error"
                    })
                    .to_string()
                )
            }
            Level::Warn => {
                println!(
                    "{}",
                    json!({
                      "ts": ts,
                      "file": file,
                      "line": line,
                      "module_path": module_path,
                      "msg": record.args().to_string(),
                      "level": "warn"
                    })
                    .to_string()
                )
            }
            Level::Info => {
                println!(
                    "{}",
                    json!({
                      "ts": ts,
                      "file": file,
                      "line": line,
                      "module_path": module_path,
                      "msg": record.args().to_string(),
                      "level": "info"
                    })
                    .to_string()
                )
            }
            Level::Debug => {}
            Level::Trace => {}
        }
    }

    fn flush(&self) {}
}

static _LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&_LOGGER)
}
