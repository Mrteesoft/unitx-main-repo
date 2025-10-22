pub mod error;
pub mod temperature;
pub mod distance;
pub mod currency;
pub mod providers;
pub mod validation;

pub use error::UnitxError;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}