//!
//! This module contains custom triggers for when to roll over a log file.
//! Currently there are the following triggers available:
//!
//! * [CustomTrigger]
//!

use log4rs::append::rolling_file::{policy::compound::trigger::Trigger, LogFile};
use std::sync::atomic::{AtomicBool, Ordering};

/// A trigger which rolls the log once it has passed a certain size
/// or the global static `LOG_FILE_EXISTS` is `true`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct CustomTrigger {
    /// The size in bytes after which the file should be rolled over.
    limit: u64,
}

impl CustomTrigger {
    /// Returns a new trigger which rolls the log once it has passed the
    /// specified size in bytes or the global static `LOG_FILE_EXISTS` is `true`.
    ///
    /// # Arguments
    ///
    /// * `limit`: the size in bytes, which triggers a roll-over if passed.
    ///
    /// returns: CustomTrigger
    ///
    /// # Examples
    ///
    /// * See [logger::default_logger](crate::logger::default_logger), line 139 for a detailed
    ///   example.

    pub fn new(limit: u64) -> CustomTrigger {
        CustomTrigger { limit }
    }
}

/// A global, thread-safe variable that tracks, if the logfile exists at program startup.
/// It is set, by [default_logger](crate::logger::default_logger) before the configuration
/// is constructed. It **should only be unset by [CustomTrigger::trigger]**, when the logfile
/// has been rolled over **ONCE** at program startup!
pub static LOG_FILE_EXISTS: AtomicBool = AtomicBool::new(false);

impl Trigger for CustomTrigger {
    /// A function that checks if the log file meets the conditions for a roll-over.
    ///
    /// The roll-over conditions are:
    ///
    /// * The size is larger than the configured limit for this instance.
    /// * The logfile exists at startup of the program (see [LOG_FILE_EXISTS]).
    ///
    /// # Arguments
    ///
    /// * `logfile`:
    ///
    /// returns: Result<bool, Error>
    fn trigger(&self, logfile: &LogFile) -> anyhow::Result<bool> {
        let log_file_exists = LOG_FILE_EXISTS.load(Ordering::Relaxed);
        let roll: bool = (logfile.len_estimate() > self.limit) || log_file_exists;

        if log_file_exists {
            LOG_FILE_EXISTS.store(false, Ordering::Relaxed);
        }

        Ok(roll)
    }
}
