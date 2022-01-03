use log4rs::append::rolling_file::{policy::compound::trigger::Trigger, LogFile};
use std::sync::{Arc, Mutex};

/// A trigger which rolls the log once it has passed a certain size
/// or the global static `LOG_FILE_EXISTS` is `true`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct CustomTrigger {
    limit: u64,
}

impl CustomTrigger {
    /// Returns a new trigger which rolls the log once it has passed the
    /// specified size in bytes or the global static `LOG_FILE_EXISTS` is `true`.
    pub fn new(limit: u64) -> CustomTrigger {
        CustomTrigger { limit }
    }
}

lazy_static!(
    pub static ref LOG_FILE_EXISTS: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
);

impl Trigger for CustomTrigger {
    fn trigger(&self, logfile: &LogFile) -> anyhow::Result<bool> {
        let mut log_file_exists = LOG_FILE_EXISTS.lock().unwrap();
        let roll: bool = (logfile.len_estimate() > self.limit) || *log_file_exists;

        if *log_file_exists {
            *log_file_exists = false;
        }

        Ok(roll)
    }
}
