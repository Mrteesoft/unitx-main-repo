pub mod error;
pub mod temperature;

pub use error::UnitxError;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}