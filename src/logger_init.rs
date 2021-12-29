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

fn default_logger(level: log::LevelFilter) -> Handle {
    // Global logging pattern
    let pattern = "{l} - {m}{n}";

    let stdout = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    // Logging policy: Roll the file over at 10MB size and keep 10 log files as {}.log.gz
    let policy = {
        CompoundPolicy::new(
            Box::new(
                SizeTrigger::new(10_000_000) // 10MB
            ),
            Box::new(
                FixedWindowRoller::builder()
                    .build("{}.log.gz", 10).unwrap()
            )
        )
    };

    // The Log file to log to and roll over if over policy size
    let logfile = RollingFileAppender::builder()
        .append(true)
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build("logs/latest.log", Box::new(policy))
        .unwrap();

    // the actual configuration
    let config = Config::builder()
        .appender(
            Appender::builder()
                .build("logfile", Box::new(logfile))
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .filter(Box::new(UpperThresholdFilter::new(LevelFilter::Info)))
                .build("stdout", Box::new(stdout)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Warn)))
                .build("stderr", Box::new(stderr))
        )
        .logger(
            Logger::builder()
                // There is no need to add the appenders here again, as this would only result in
                // duplicate log entries.
                .build("xd_bot", level)
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .appender("stderr")
                .build(LevelFilter::Warn),
        )
        .unwrap();

    log4rs::init_config(config).unwrap()
}

#[cfg(debug_assertions)]
pub fn init() -> Handle {
    default_logger(log::LevelFilter::Debug)
}

#[cfg(not(debug_assertions))]
pub fn init() -> Handle {
    default_logger(log::LevelFilter::Info)
}
