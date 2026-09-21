use thiserror::Error;

/// Stable error type for SVG processing.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("Input SVG is empty")]
    EmptyInput,

    #[error("Failed to process SVG content: {0}")]
    ParseError(String),

    #[error("Precision must be between 0 and 6, got {0}")]
    InvalidPrecision(u32),

    #[error("Failed to encode SVG as data URI")]
    DataUriEncode,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl CoreError {
    /// Returns a stable machine error code for Web, CLI, and Agent consumers.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::EmptyInput => "EMPTY_INPUT",
            Self::ParseError(_) => "PARSE_ERROR",
            Self::InvalidPrecision(_) => "INVALID_PRECISION",
            Self::DataUriEncode => "DATA_URI_ENCODE",
            Self::InternalError(_) => "INTERNAL_ERROR",
        }
    }
}
