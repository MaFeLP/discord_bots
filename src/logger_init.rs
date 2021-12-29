use log4rs::{
    append::{
        console::{
            ConsoleAppender,
            Target
        },
        rolling_file::{
            policy::compound::{
                CompoundPolicy,
                roll::fixed_window::FixedWindowRoller,
                trigger::size::SizeTrigger,
            },
            RollingFileAppender,
        },
    },
    config::{
        Appender,
        Logger,
        Root,
    },
    encode::pattern::PatternEncoder,
    filter::{
        Filter,
        threshold::ThresholdFilter,
        Response,
    },
    {
        Config,
        Handle
    },
};
use log::{
    LevelFilter,
    Record
};

/// A filter that rejects all events at a level above a provided threshold.
///
/// Upper Threshold implemented here, as log4rs doesn't have it.
/// This is copied from [the official documentation](https://docs.rs/log4rs/latest/src/log4rs/filter/threshold.rs.html#20-22)
/// with `>` in the function `filter` changed to `<`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct UpperThresholdFilter {
    level: LevelFilter,
}

impl UpperThresholdFilter {
    /// Creates a new `ThresholdFilter` with the specified maximum logging
    pub fn new(level: LevelFilter) -> UpperThresholdFilter {
        UpperThresholdFilter { level }
    }
}

impl Filter for UpperThresholdFilter {
    fn filter(&self, record: &Record) -> Response {
        // Changed from `>` to `<`.
        if record.level() < self.level {
            Response::Reject
        } else {
            Response::Neutral
        }
    }
}

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
fn default_logger(level: log::LevelFilter) -> Handle {
    // Global logging pattern
    // TODO make pattern configurable in the config file
    let pattern = "{h({d(%Y-%m-%d %H:%M:%S)} [{t}/{l}]: {m:>10.15}{n})}";

    // STDOUT and STDERR with the specified pattern
    let stdout = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    // Log file policy: Roll the file over at 10MB size and keep 10 log files as {}.log.gz
    let policy = {
        CompoundPolicy::new(
            Box::new(
                // TODO make rollover size configurable in the config file
                SizeTrigger::new(10_000_000) // 10MB
            ),
            Box::new(
                FixedWindowRoller::builder()
                    // TODO make file pattern size configurable in the config file
                    // TODO make log file count configurable in the config file
                    .build("logs/{}.log.gz", 10).unwrap()
            )
        )
    };

    // The Log file to log to and roll over if over policy size
    let logfile = RollingFileAppender::builder()
        .append(true)
        .encoder(Box::new(PatternEncoder::new(pattern)))
        // TODO make file pattern size configurable in the config file
        .build("logs/latest.log", Box::new(policy))
        .unwrap();

    // the actual configuration
    let config = Config::builder()
        // Add the logfile appender
        .appender(
            Appender::builder()
                .build("logfile", Box::new(logfile))
        )

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
                .build("stderr", Box::new(stderr))
        )

        // Build the normal logger and configure it with the minimum log level for debug/release builds
        .logger(
            Logger::builder()
                // There is no need to add the appenders here again, as this would only result in
                // duplicate log entries.
                .build("xd_bot", level)
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
    log4rs::init_config(config).unwrap()
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
pub fn init() -> Handle {
    default_logger(log::LevelFilter::Debug)
}

#[cfg(not(debug_assertions))]
/// A wrapper function for [logger_init::default_logger](crate::logger_init::default_logger).
/// For release builds the log level is set down to INFO.
///
/// returns: Handle
///
/// # Examples
///
/// ```
/// logger_init::init();
/// ```
pub fn init() -> Handle {
    // TODO make threshold log level configurable via environment
    default_logger(log::LevelFilter::Info)
}
