//! Error types for sankhya.
//!
//! All errors are returned via `Result` — no panics in library code.

use serde::{Deserialize, Serialize};

/// Errors that can occur in sankhya computations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[non_exhaustive]
pub enum SankhyaError {
    /// A date value is out of the valid range for the calendar system.
    #[error("invalid date: {0}")]
    InvalidDate(String),

    /// A number base or digit value is out of range.
    #[error("invalid base: {0}")]
    InvalidBase(String),

    /// A fraction is invalid (e.g., zero denominator).
    #[error("invalid fraction: {0}")]
    InvalidFraction(String),

    /// An arithmetic overflow occurred.
    #[error("overflow: {0}")]
    OverflowError(String),

    /// A computation failed to converge or produced an invalid result.
    #[error("computation error: {0}")]
    ComputationError(String),
}

/// Convenience alias for results in this crate.
pub type Result<T> = core::result::Result<T, SankhyaError>;
