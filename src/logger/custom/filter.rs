//!
//! This module contains all custom logging filters. A filter examines a record which is to be
//! logged and then decides, if it should be logged or not.
//!
//! Current filters available:
//!
//! * [UpperThresholdFilter]
//!

use log::{LevelFilter, Record};
use log4rs::filter::{Filter, Response};

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
    /// Creates a new `UpperThresholdFilter` with the specified maximum logging
    ///
    /// # Arguments
    ///
    /// * `level`: The level **above** which this filter should reject the log messages.
    ///
    /// returns: UpperThresholdFilter
    ///
    /// # Examples
    /// * See [logger::default_logger](crate::logger::default_logger), line 139 for a detailed
    ///   example.
    pub fn new(level: LevelFilter) -> UpperThresholdFilter {
        UpperThresholdFilter { level }
    }
}

impl Filter for UpperThresholdFilter {
    /// The filter function which holds the logic, whether the record should be rejected.
    ///
    /// # Arguments
    ///
    /// * `record`: The log record which is tested for rejection.
    ///
    /// returns: Response
    fn filter(&self, record: &Record) -> Response {
        // Changed from `>` to `<`.
        if record.level() < self.level {
            Response::Reject
        } else {
            Response::Neutral
        }
    }
}
