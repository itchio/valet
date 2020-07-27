use flume::TrySendError;
use napi::ToNapi;
use std::{sync::Mutex, time::SystemTime};

// A logger sending records to a channel
pub struct Logger {
    pub log_to_stderr: bool,
    pub sender: Mutex<flume::Sender<Record>>,
}

#[derive(Debug)]
pub struct Record {
    level: log::Level,
    target: String,
    time: u64,
    message: String,
}

impl ToNapi for Record {
    fn to_napi(&self, env: &napi::JsEnv) -> napi::JsResult<napi::RawValue> {
        let obj = env.object()?;
        obj.set_property(
            "level",
            match self.level {
                log::Level::Error => "error",
                log::Level::Warn => "warn",
                log::Level::Info => "info",
                log::Level::Debug => "debug",
                log::Level::Trace => "trace",
            },
        )?;
        obj.set_property("time", self.time as i64)?;
        obj.set_property("message", self.message.as_str())?;
        obj.set_property("target", self.target.as_str())?;
        Ok(obj.value)
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // TODO: actually filter some stuff
        true
    }
    fn log(&self, record: &log::Record) {
        if self.log_to_stderr {
            eprintln!(
                "{}:{} -- {}",
                record.level(),
                record.target(),
                record.args()
            );
        }

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let record = Record {
            target: record.target().to_string(),
            level: record.level(),
            time,
            message: format!("{}", record.args()),
        };

        let sender = self.sender.lock().unwrap();
        if let Err(err) = sender.try_send(record) {
            match err {
                TrySendError::Full(record) => eprintln!("Dropping log record (full): {:?}", record),
                TrySendError::Disconnected(record) => {
                    eprintln!("Dropping log record (disconnected): {:?}", record)
                }
            }
        }
    }
    fn flush(&self) {
        // muffin to do
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        eprintln!("=== LOGGER IS BEING DROPPED ===");
    }
}
