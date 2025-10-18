use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnitxError {
    #[error("invalid unit: {0}")]
    InvalidUnit(String),
    #[error("conversion not supported: {0} -> {1}")]
    NotSupported(String, String),
}