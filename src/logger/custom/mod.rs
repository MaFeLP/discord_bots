//!
//! This module contains custom components for the log4rs configuration,
//! as the default components do not have the needed functionality.
//!
//! Current components include:
//!
//! * [filter::UpperThresholdFilter]
//! * [trigger::CustomTrigger]
//!

pub mod filter;
pub mod trigger;
