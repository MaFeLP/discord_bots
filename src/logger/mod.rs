//!
//! The logging module which handles all the configuration of
//! the [log4rs] instance for this program.
//!
//! After calling its [init] methods, the program then can use
//! [log]'s macros to log entries to three different outputs:
//!
//! 1. Everything is logged into a log file, located by default
//!    at `logs/latest.log`.
//! 2. Depending on the log level, the response is also logged to stdout or stderr:
//!     * All logs above or at level of `WARN`, will **always** be logged to stderr.
//!     * All logs below `WARN` will be logged to stdout. This can be configured on
//!       release builds with the `LOGGING_LEVEL_THRESHOLD` environment variable.
//!
//! The logger's default pattern is:
//! ```log
//! YYYY-mm-dd HH:MM:SS [Module/LEVEL]: Message
//! ```
//!

use crate::logger::custom::{
    filter::UpperThresholdFilter,
    trigger::{CustomTrigger, LOG_FILE_EXISTS},
};
use log::{trace, warn, LevelFilter};
use log4rs::config::Deserializers;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::{
            policy::compound::{roll::fixed_window::FixedWindowRoller, CompoundPolicy},
            RollingFileAppender,
        },
    },
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};
use std::path::Path;
use std::{env, fs};
use std::sync::atomic::Ordering;

mod custom;

/// A function that initializes the global logger.
///
/// # Arguments
///
/// * `level`: The minimum level to log with: DEBUG/TRACE for debug builds and INFO for releases.
///
/// returns: Handle
///
/// # Examples
///
/// ```
/// default_logger(LevelFilter::Debug);
/// ```
fn default_logger(level: log::LevelFilter) {
    let mut warnings: Vec<String> = Vec::new();

    // Give the user to specify their own logging file
    match env::var("LOGGING_CONFIG_FILE") {
        Ok(s) => {
            if Path::new(&s).exists() {
                match log4rs::init_file(&s, Deserializers::default()) {
                    Ok(_) => {
                        warn!("Using custom logger configuration at: {}", s);
                        trace!("Config contents:\n{}", fs::read_to_string(&s).unwrap());
                        return;
                    }
                    Err(why) => {
                        warnings.push(format!(
                            "\"{}\" is not a valid config file. Using defaults!",
                            s
                        ));
                        warnings.push(format!("Error message: {}", why));
                    }
                }
            }
        }
        _ => {}
    }

    // Get changeable logger attributes from environment
    // Why environment? Because environments are more easily configurable in docker containers
    // than command line options and the config file is read in later to use this logger.

    // Global logging pattern
    let pattern = match env::var("LOGGING_PATTERN") {
        Ok(s) => {
            warnings.push(format!("Logging pattern has been overridden to: {}", s));
            s
        }
        Err(_) => String::from("{h({d(%Y-%m-%d %H:%M:%S)} [{t}/{l}]: {m:>10}{n})}"), // Default
    };
    // Rollover Size
    let rollover_size = match env::var("LOGGING_ROLLOVER_SIZE") {
        Ok(s) => match s.parse::<u64>() {
            Ok(r) => {
                warnings.push(format!(
                    "Log file rollover size has been overridden to: {}",
                    r
                ));
                r
            }
            Err(_) => {
                warnings.push(format!(
                    "{} is not a valid number! Defaulting to 10,000,000",
                    s
                ));
                10_000_000
            }
        },
        Err(_) => 10_000_000, // Default
    };
    // The logging folder
    let folder = match env::var("LOGGING_FOLDER") {
        Ok(s) => {
            warnings.push(format!("Logging folder has been overridden to: {}", s));
            s
        }
        Err(_) => String::from("logs"), // Default
    };
    // The log archive pattern
    let archive_pattern = match env::var("LOGGING_ARCHIVE_PATTERN") {
        Ok(s) => {
            warnings.push(format!(
                "Logging archive pattern has been overridden to: {}",
                s
            ));
            s
        }
        Err(_) => String::from("{}.log.gz"), // Default
    };
    // Rollover Size
    let log_file_count = match env::var("LOGGING_FILE_COUNT") {
        Ok(s) => match s.parse::<u32>() {
            Ok(r) => {
                warnings.push(format!("Log file count has been overridden to: {}", r));
                r
            }
            Err(_) => {
                warnings.push(format!("{} is not a valid number! Defaulting to 10", s));
                10
            }
        },
        Err(_) => 10, // Default
    };

    // STDOUT and STDERR with the specified pattern
    let stdout = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new(&pattern)))
        .build();
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(&pattern)))
        .build();

    // Log file policy: Roll the file over at 10MB size and keep 10 log files as {}.log.gz
    let policy = {
        CompoundPolicy::new(
            Box::new(
                CustomTrigger::new(rollover_size), // 10MB
            ),
            Box::new(
                FixedWindowRoller::builder()
                    .build(
                        &format!("{}/{}", folder, archive_pattern),
                        log_file_count,
                    )
                    .unwrap(),
            ),
        )
    };

    // The Log file to log to and roll over if over policy size
    let log_file_path = format!("{}/latest.log", folder);
    if Path::new(&log_file_path).exists() {
        // If it exists, roll over before first log entry.
        LOG_FILE_EXISTS.store(true, Ordering::Relaxed);
    }
    let logfile = RollingFileAppender::builder()
        .append(true)
        .encoder(Box::new(PatternEncoder::new(&pattern)))
        .build(Path::new(&log_file_path), Box::new(policy))
        .unwrap();

    // the actual configuration
    let config = Config::builder()
        // Add the logfile appender
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        // Add the stdout appender to log all messages between two thresholds:
        // Minimum: DEBUG/TRACE on debug builds and INFO on release builds
        // Maximum: INFO
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .filter(Box::new(UpperThresholdFilter::new(LevelFilter::Info)))
                .build("stdout", Box::new(stdout)),
        )
        // Add the stderr appender to log all errors and warns to.
        // This ensures that the admin will get notified about errors, even if they pipe stdout to stderr.
        // Minimum: WARN
        // Maximum: N/A
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Warn)))
                .build("stderr", Box::new(stderr)),
        )
        // Build the normal logger and configure it with the minimum log level for debug/release builds
        .logger(
            Logger::builder()
                // There is no need to add the appenders here again, as this would only result in
                // duplicate log entries.
                .build("xd_bot", level),
        )
        // Configure the default logger
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .appender("stderr")
                // The minimum level is set to WARN, so dependencies do not spam the logs with their
                // "uninteresting" logs, but can still send warnings.
                .build(LevelFilter::Warn),
        )
        .unwrap();

    // Initialize the configuration and create the global logger.
    log4rs::init_config(config).unwrap();

    for msg in warnings {
        warn!(target: "xd_bot", "{}", msg);
    }
}

#[cfg(debug_assertions)]
/// A wrapper function for [logger_init::default_logger](crate::logger_init::default_logger).
/// For debug builds the log level is set down to DEBUG.
///
/// returns: Handle
///
/// # Examples
///
/// ```
/// logger_init::init();
/// ```
pub fn init() {
    default_logger(log::LevelFilter::Debug)
}

#[cfg(not(debug_assertions))]
/// A wrapper function for [logger_init::default_logger](crate::logger_init::default_logger).
/// For release builds the log level is set down to INFO, if not set via the environment.
///
/// returns: Handle
///
/// # Examples
///
/// ```
/// logger_init::init();
/// ```
pub fn init() {
    use log::LevelFilter::*;

    let logging_level = match env::var("LOGGING_LEVEL_THRESHOLD") {
        Ok(s) => match s.to_ascii_lowercase().as_str() {
            "trace" => Trace,
            "debug" => Debug,
            "info" => Info,
            "warn" => Warn,
            "error" => Error,
            _ => Info,
        },
        Err(_) => Info,
    };
    let handle = default_logger(logging_level);

    if logging_level != Info {
        log::warn!(
            "Default logging level has been overridden to: {}",
            logging_level
        );
    }

    handle
}
