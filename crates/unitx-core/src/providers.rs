use crate::currency::CurrencyUnit;
use rust_decimal::Decimal;

pub mod live;

pub use live::LiveExchangeProvider;

pub trait ExchangeRateProvider {
    fn get_rate(&self, from: CurrencyUnit, to: CurrencyUnit) -> Result<Decimal, String>;
}
