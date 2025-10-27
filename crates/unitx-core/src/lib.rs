pub mod currency;
pub mod distance;
pub mod error;
pub mod providers;
pub mod temperature;
pub mod validation;

pub use error::UnitxError;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
