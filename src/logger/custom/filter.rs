use log4rs::filter::{Filter, Response};
use log::{LevelFilter, Record};

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

