use crate::providers::ExchangeRateProvider;
use crate::providers::LiveExchangeProvider;
use rust_decimal::Decimal;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CurrencyUnit {
    USD,
    EUR,
    GBP,
    JPY,
}

impl CurrencyUnit {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "USD" => Some(Self::USD),
            "EUR" => Some(Self::EUR),
            "GBP" => Some(Self::GBP),
            "JPY" => Some(Self::JPY),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::USD => "USD",
            Self::EUR => "EUR",
            Self::GBP => "GBP",
            Self::JPY => "JPY",
        }
    }
}

impl fmt::Display for CurrencyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub fn convert_with_provider<P: ExchangeRateProvider>(
    value: Decimal,
    from: CurrencyUnit,
    to: CurrencyUnit,
    provider: &P,
) -> Result<Decimal, String> {
    let rate = provider.get_rate(from, to)?;
    Ok(value * rate)
}

/// Convenience helper that fetches live rates directly.
///
/// This performs a blocking HTTP call using the default live provider. For deterministic or
/// offline scenarios prefer `convert_with_provider` so you can supply your own rate source.
pub fn convert(value: Decimal, from: CurrencyUnit, to: CurrencyUnit) -> Result<Decimal, String> {
    let provider = LiveExchangeProvider::new(None);
    convert_with_provider(value, from, to, &provider)
}
